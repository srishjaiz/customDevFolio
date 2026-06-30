//! Stream CSV → NDJSON on disk for API uploads (Phase 5). Free `csv` crate.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::error::{RepoError, RepoResult};
use crate::slug::{is_valid_slug, normalize_slug};

#[derive(Debug, Default)]
pub struct ConvertStats {
    pub total_rows: u64,
    pub succeeded: u64,
    pub failed: u64,
}

pub fn csv_file_to_ndjson(
    csv_path: &Path,
    ndjson_path: &Path,
    errors_path: Option<&Path>,
) -> RepoResult<ConvertStats> {
    let input = File::open(csv_path).map_err(RepoError::Io)?;
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(input);

    if let Some(parent) = ndjson_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(RepoError::Io)?;
        }
    }
    let out = File::create(ndjson_path).map_err(RepoError::Io)?;
    let mut writer = BufWriter::new(out);

    let mut err_w = if let Some(p) = errors_path {
        let f = File::create(p).map_err(RepoError::Io)?;
        Some(BufWriter::new(f))
    } else {
        None
    };

    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| RepoError::Invalid(e.to_string()))?
        .iter()
        .map(|h| h.trim().to_ascii_lowercase().replace(['-', ' '], "_"))
        .map(|h| match h.as_str() {
            "full_name" | "person_name" => "name".into(),
            "primary_color" => "primary".into(),
            "theme_mode" => "theme".into(),
            other => other.to_string(),
        })
        .collect();

    let mut stats = ConvertStats::default();
    let mut seen: HashMap<String, u64> = HashMap::new();

    for (idx, rec) in reader.records().enumerate() {
        let line_no = (idx + 2) as u64;
        stats.total_rows += 1;
        let rec = match rec {
            Ok(r) => r,
            Err(e) => {
                stats.failed += 1;
                write_err(&mut err_w, line_no, &e.to_string())?;
                continue;
            }
        };
        let mut fields = HashMap::new();
        for (i, h) in headers.iter().enumerate() {
            if let Some(v) = rec.get(i) {
                if !v.is_empty() {
                    fields.insert(h.clone(), v.to_string());
                }
            }
        }
        let name = match fields.get("name") {
            Some(n) if !n.is_empty() => n.clone(),
            _ => {
                stats.failed += 1;
                write_err(&mut err_w, line_no, "missing name")?;
                continue;
            }
        };
        let domain = fields
            .get("domain")
            .cloned()
            .unwrap_or_else(|| "fullstack".into());
        let mut slug = fields
            .get("slug")
            .cloned()
            .unwrap_or_else(|| normalize_slug(&name));
        if !is_valid_slug(&slug) {
            slug = normalize_slug(&slug);
        }
        let c = seen.entry(slug.clone()).or_insert(0);
        *c += 1;
        if *c > 1 {
            slug = format!("{slug}-{}", *c);
        }
        let title = fields
            .get("title")
            .cloned()
            .unwrap_or_else(|| "Software Engineer".into());
        let bio = fields
            .get("bio")
            .cloned()
            .unwrap_or_else(|| format!("Portfolio for {name}."));
        let primary = fields
            .get("primary")
            .cloned()
            .unwrap_or_else(|| "#6366f1".into());
        let theme = fields
            .get("theme")
            .cloned()
            .unwrap_or_else(|| "system".into());
        let email = fields.get("email").cloned();
        let github = fields.get("github").cloned();

        let mut person = serde_json::json!({
            "name": name,
            "title": title,
            "bio": bio,
        });
        if let Some(loc) = fields.get("location") {
            person["location"] = serde_json::json!(loc);
        }
        if let Some(r) = fields.get("resume_url") {
            person["resumeUrl"] = serde_json::json!(r);
        }

        let mut social = serde_json::Map::new();
        if let Some(e) = email {
            social.insert("email".into(), serde_json::json!(e));
        }
        if let Some(g) = github {
            let url = if g.starts_with("http") {
                g
            } else {
                format!("https://github.com/{g}")
            };
            social.insert("github".into(), serde_json::json!(url));
        }
        if let Some(l) = fields.get("linkedin") {
            social.insert("linkedin".into(), serde_json::json!(l));
        }
        if let Some(w) = fields.get("website") {
            social.insert("website".into(), serde_json::json!(w));
        }

        let doc = serde_json::json!({
            "slug": slug,
            "meta": {
                "title": format!("{name} — {title}"),
                "description": bio,
            },
            "domain": domain,
            "person": person,
            "social": social,
            "greeting": {
                "headline": format!("Hi, I'm {name}."),
                "subheadline": title,
            },
            "skills": { "title": "Skills", "groups": [] },
            "experience": [],
            "education": [],
            "projects": [],
            "achievements": [],
            "blog": { "enabled": false, "posts": [] },
            "contact": { "title": "Contact", "email": fields.get("email") },
            "theme": { "primary": primary, "mode": theme },
            "sections": {
                "skills": true,
                "experience": true,
                "projects": true,
                "education": true,
                "achievements": true,
                "blog": false,
                "contact": true
            }
        });
        serde_json::to_writer(&mut writer, &doc)
            .map_err(|e| RepoError::Invalid(e.to_string()))?;
        writer.write_all(b"\n").map_err(RepoError::Io)?;
        stats.succeeded += 1;
    }
    writer.flush().map_err(RepoError::Io)?;
    if let Some(w) = err_w.as_mut() {
        w.flush().map_err(RepoError::Io)?;
    }
    Ok(stats)
}

fn write_err(
    err_w: &mut Option<BufWriter<File>>,
    line: u64,
    msg: &str,
) -> RepoResult<()> {
    if let Some(w) = err_w {
        let o = serde_json::json!({"line": line, "error": msg});
        serde_json::to_writer(&mut *w, &o).map_err(|e| RepoError::Invalid(e.to_string()))?;
        w.write_all(b"\n").map_err(RepoError::Io)?;
    }
    Ok(())
}
