//! Stream large CSV portfolio datasets to NDJSON on disk (Phase 2).
//!
//! Never loads the full CSV into memory as one string or `Vec` of all records.
//! Free OSS: `csv` + `serde_json` crates. See `docs/adr/0001-free-stack.md`.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::Serialize;

use crate::config::{DomainId, PortfolioConfig, ThemeMode};
use crate::domain::CreateAnswers;
use crate::slug_util::{is_valid_slug, normalize_slug};

/// One NDJSON line: portfolio config plus multi-folio `slug`.
#[derive(Debug, Clone, Serialize)]
pub struct NdjsonPortfolioRecord {
    pub slug: String,
    #[serde(flatten)]
    pub config: PortfolioConfig,
}

#[derive(Debug, Clone, Default)]
pub struct ConvertOptions {
    /// When true, incomplete rows use sample experience/projects (like `create` without `--minimal`).
    pub include_sample_content: bool,
    /// Continue after row errors; still returns Ok with failed counts.
    pub continue_on_error: bool,
    /// Optional path for per-row error lines (`{"line":N,"error":"…"}`).
    pub errors_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConvertStats {
    pub total_rows: u64,
    pub succeeded: u64,
    pub failed: u64,
}

/// Stream `input` CSV → write `output` NDJSON (one portfolio JSON object per line).
pub fn csv_to_ndjson(input: &Path, output: &Path, opts: &ConvertOptions) -> Result<ConvertStats> {
    let input_file = File::open(input).with_context(|| format!("open CSV {}", input.display()))?;
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(input_file);

    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create output dir {}", parent.display()))?;
        }
    }
    let out_file =
        File::create(output).with_context(|| format!("create NDJSON {}", output.display()))?;
    let mut writer = BufWriter::new(out_file);

    let mut err_writer = if let Some(ref err_path) = opts.errors_path {
        if let Some(parent) = err_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let f = File::create(err_path)
            .with_context(|| format!("create errors file {}", err_path.display()))?;
        Some(BufWriter::new(f))
    } else {
        None
    };

    let headers = reader
        .headers()
        .context("read CSV headers")?
        .iter()
        .map(normalize_header)
        .collect::<Vec<_>>();

    if headers.is_empty() {
        bail!("CSV has no headers");
    }

    let mut stats = ConvertStats::default();
    // Track slugs seen in this file to avoid duplicates (in-memory set of strings only, not full rows).
    let mut seen_slugs: HashMap<String, u64> = HashMap::new();

    for (idx, result) in reader.records().enumerate() {
        let line_no = (idx + 2) as u64; // header is line 1
        stats.total_rows += 1;

        let record = match result {
            Ok(r) => r,
            Err(e) => {
                stats.failed += 1;
                write_row_error(&mut err_writer, line_no, &e.to_string())?;
                if opts.continue_on_error {
                    continue;
                }
                bail!("CSV parse error at line {line_no}: {e}");
            }
        };

        let mut fields: HashMap<String, String> = HashMap::new();
        for (i, key) in headers.iter().enumerate() {
            if let Some(val) = record.get(i) {
                if !val.is_empty() {
                    fields.insert(key.clone(), val.to_string());
                }
            }
        }

        match row_to_record(&fields, opts.include_sample_content, &mut seen_slugs) {
            Ok(rec) => {
                serde_json::to_writer(&mut writer, &rec)
                    .with_context(|| format!("serialize row at line {line_no}"))?;
                writer.write_all(b"\n")?;
                stats.succeeded += 1;
            }
            Err(e) => {
                stats.failed += 1;
                write_row_error(&mut err_writer, line_no, &e.to_string())?;
                if !opts.continue_on_error {
                    bail!("row error at line {line_no}: {e}");
                }
            }
        }
    }

    writer.flush().context("flush NDJSON")?;
    if let Some(ref mut ew) = err_writer {
        ew.flush().context("flush errors file")?;
    }

    Ok(stats)
}

fn write_row_error(
    err_writer: &mut Option<BufWriter<File>>,
    line_no: u64,
    message: &str,
) -> Result<()> {
    if let Some(w) = err_writer {
        let obj = serde_json::json!({ "line": line_no, "error": message });
        serde_json::to_writer(&mut *w, &obj)?;
        w.write_all(b"\n")?;
    }
    Ok(())
}

/// Normalize header names to a small canonical set.
fn normalize_header(raw: &str) -> String {
    let s = raw
        .trim()
        .to_ascii_lowercase()
        .replace(['-', ' '], "_");
    match s.as_str() {
        "full_name" | "fullname" | "person_name" | "displayname" | "display_name" => "name".into(),
        "primary_color" | "primarycolor" | "color" => "primary".into(),
        "theme_mode" | "thememode" | "color_mode" => "theme".into(),
        "resumeurl" | "resume" => "resume_url".into(),
        "site" | "site_url" | "siteurl" => "website".into(),
        "extra" | "json" | "config_json" | "portfolio_json" => "extra_json".into(),
        other => other.to_string(),
    }
}

fn field<'a>(fields: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    fields
        .get(key)
        .map(|s| s.as_str())
        .filter(|s| !s.is_empty())
}

fn row_to_record(
    fields: &HashMap<String, String>,
    include_sample_content: bool,
    seen_slugs: &mut HashMap<String, u64>,
) -> Result<NdjsonPortfolioRecord> {
    let name = field(fields, "name")
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("missing required column `name` (or full_name)"))?;

    let domain = match field(fields, "domain") {
        Some(d) => d.parse::<DomainId>().map_err(|e| anyhow::anyhow!("{e}"))?,
        None => DomainId::Fullstack,
    };

    let mut slug = field(fields, "slug")
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_else(|| normalize_slug(&name));
    if !is_valid_slug(&slug) {
        slug = normalize_slug(&slug);
    }
    if !is_valid_slug(&slug) {
        bail!("invalid slug derived from name/slug");
    }
    // Disambiguate duplicates within the same CSV.
    let count = seen_slugs.entry(slug.clone()).or_insert(0);
    *count += 1;
    if *count > 1 {
        slug = format!("{slug}-{}", *count);
    }

    let theme_mode = match field(fields, "theme") {
        Some(t) => t.parse::<ThemeMode>().map_err(|e| anyhow::anyhow!("{e}"))?,
        None => ThemeMode::System,
    };

    let answers = CreateAnswers {
        project_name: slug.clone(),
        domain,
        display_name: name,
        title: field(fields, "title").map(|s| s.to_string()),
        bio: field(fields, "bio").map(|s| s.to_string()),
        location: field(fields, "location").map(|s| s.to_string()),
        email: field(fields, "email").map(|s| s.to_string()),
        github: field(fields, "github").map(|s| s.to_string()),
        linkedin: field(fields, "linkedin").map(|s| s.to_string()),
        website: field(fields, "website").map(|s| s.to_string()),
        resume_url: field(fields, "resume_url").map(|s| s.to_string()),
        primary_color: field(fields, "primary").map(|s| s.to_string()),
        theme_mode,
        include_sample_content,
    };

    let mut config = answers.into_portfolio();

    // Optional full/partial JSON overlay (merged shallowly over generated config).
    if let Some(extra) = field(fields, "extra_json") {
        let overlay: serde_json::Value =
            serde_json::from_str(extra).context("parse extra_json column")?;
        let mut base = serde_json::to_value(&config).context("serialize portfolio for merge")?;
        merge_json(&mut base, &overlay);
        config = serde_json::from_value(base).context("deserialize merged portfolio")?;
    }

    Ok(NdjsonPortfolioRecord { slug, config })
}

/// Shallow-recursive object merge: overlay keys replace/merge into base.
fn merge_json(base: &mut serde_json::Value, overlay: &serde_json::Value) {
    match (base, overlay) {
        (serde_json::Value::Object(base_map), serde_json::Value::Object(over_map)) => {
            for (k, v) in over_map {
                match base_map.get_mut(k) {
                    Some(bv) if bv.is_object() && v.is_object() => merge_json(bv, v),
                    _ => {
                        base_map.insert(k.clone(), v.clone());
                    }
                }
            }
        }
        (b, o) => *b = o.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn normalize_headers() {
        assert_eq!(normalize_header("Full Name"), "name");
        assert_eq!(normalize_header("primaryColor"), "primary");
        assert_eq!(normalize_header("theme_mode"), "theme");
        assert_eq!(normalize_header("resumeUrl"), "resume_url");
    }

    #[test]
    fn converts_small_csv_streaming() {
        let dir = tempdir().unwrap();
        let csv_path = dir.path().join("people.csv");
        let ndjson_path = dir.path().join("people.ndjson");
        let mut f = File::create(&csv_path).unwrap();
        writeln!(
            f,
            "slug,domain,name,title,email,github\nada-lovelace,frontend,Ada Lovelace,Frontend Engineer,ada@example.com,ada\ngrace-hopper,backend,Grace Hopper,Backend Engineer,grace@example.com,grace"
        )
        .unwrap();

        let stats = csv_to_ndjson(
            &csv_path,
            &ndjson_path,
            &ConvertOptions {
                include_sample_content: false,
                continue_on_error: false,
                errors_path: None,
            },
        )
        .unwrap();
        assert_eq!(stats.total_rows, 2);
        assert_eq!(stats.succeeded, 2);
        assert_eq!(stats.failed, 0);

        let text = std::fs::read_to_string(&ndjson_path).unwrap();
        let lines: Vec<_> = text.lines().filter(|l| !l.is_empty()).collect();
        assert_eq!(lines.len(), 2);
        let first: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(first["slug"], "ada-lovelace");
        assert_eq!(first["domain"], "frontend");
        assert_eq!(first["person"]["name"], "Ada Lovelace");
        assert!(first.get("skills").is_some());
    }

    #[test]
    fn continues_on_error_and_writes_errors_file() {
        let dir = tempdir().unwrap();
        let csv_path = dir.path().join("bad.csv");
        let ndjson_path = dir.path().join("out.ndjson");
        let err_path = dir.path().join("errors.ndjson");
        let mut f = File::create(&csv_path).unwrap();
        // Second row missing name
        writeln!(f, "domain,name\nfrontend,Ok Person\nbackend,").unwrap();

        let stats = csv_to_ndjson(
            &csv_path,
            &ndjson_path,
            &ConvertOptions {
                include_sample_content: false,
                continue_on_error: true,
                errors_path: Some(err_path.clone()),
            },
        )
        .unwrap();
        assert_eq!(stats.succeeded, 1);
        assert_eq!(stats.failed, 1);
        let err_text = std::fs::read_to_string(&err_path).unwrap();
        assert!(err_text.contains("missing required"));
    }

    #[test]
    fn disambiguates_duplicate_slugs() {
        let dir = tempdir().unwrap();
        let csv_path = dir.path().join("dup.csv");
        let ndjson_path = dir.path().join("out.ndjson");
        let mut f = File::create(&csv_path).unwrap();
        writeln!(f, "name\nAda Lovelace\nAda Lovelace").unwrap();

        let stats = csv_to_ndjson(
            &csv_path,
            &ndjson_path,
            &ConvertOptions {
                include_sample_content: false,
                continue_on_error: false,
                errors_path: None,
            },
        )
        .unwrap();
        assert_eq!(stats.succeeded, 2);
        let text = std::fs::read_to_string(&ndjson_path).unwrap();
        let lines: Vec<_> = text.lines().collect();
        let a: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
        let b: serde_json::Value = serde_json::from_str(lines[1]).unwrap();
        assert_eq!(a["slug"], "ada-lovelace");
        assert_eq!(b["slug"], "ada-lovelace-2");
    }

    #[test]
    fn rejects_unknown_domain_without_continue() {
        let dir = tempdir().unwrap();
        let csv_path = dir.path().join("bad-domain.csv");
        let ndjson_path = dir.path().join("out.ndjson");
        let mut f = File::create(&csv_path).unwrap();
        writeln!(f, "name,domain\nAda,not-a-domain").unwrap();
        let err = csv_to_ndjson(
            &csv_path,
            &ndjson_path,
            &ConvertOptions {
                include_sample_content: false,
                continue_on_error: false,
                errors_path: None,
            },
        );
        assert!(err.is_err());
    }
}
