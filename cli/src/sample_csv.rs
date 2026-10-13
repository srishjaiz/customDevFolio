//! Generate sample multi-portfolio CSVs for local testing (streaming writer).

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{bail, Context, Result};

/// Header row matching `examples/import` column contract.
pub const SAMPLE_CSV_HEADER: &str =
    "slug,domain,name,title,bio,location,email,github,linkedin,website,resume_url,primary,theme";

/// Built-in demo people (happy path, multiple domains).
const DEMO_ROWS: &[&str] = &[
    "ada-lovelace,frontend,Ada Lovelace,Frontend Engineer,Building accessible UI systems.,London,ada@example.com,ada,https://linkedin.com/in/ada,https://ada.dev,https://ada.dev/cv.pdf,#ec4899,system",
    "grace-hopper,backend,Grace Hopper,Backend Engineer,APIs and reliable services.,Remote,grace@example.com,grace,https://linkedin.com/in/grace,https://grace.dev,,#3b82f6,dark",
    "alan-turing,ml,Alan Turing,ML Engineer,Models and evaluation.,Cambridge,alan@example.com,alan,,,https://alan.dev/resume.pdf,#0d9488,system",
    "margaret-hamilton,fullstack,Margaret Hamilton,Full-Stack Engineer,Software that ships to the moon.,Boston,margaret@example.com,mhamilton,https://linkedin.com/in/mhamilton,https://hamilton.dev,,#6366f1,light",
    "linus-torvalds,devops,Linus Torvalds,Platform Engineer,Kernels CI and infrastructure.,Helsinki,linus@example.com,torvalds,,,https://kernel.org,#f97316,dark",
    "radia-perlman,security,Radia Perlman,Security Engineer,Network protocols and trust boundaries.,Remote,radia@example.com,rperlman,https://linkedin.com/in/radia,,,#ef4444,system",
    "donald-knuth,data,Donald Knuth,Data Engineer,Algorithms and data pipelines.,Stanford,knuth@example.com,knuth,,,https://www-cs-faculty.stanford.edu/~knuth/,#8b5cf6,system",
    "guido-van-rossum,backend,Guido van Rossum,Backend Engineer,Readable APIs and services.,Remote,guido@example.com,gvanrossum,https://linkedin.com/in/guido,,,#3776ab,light",
    "brendan-eich,frontend,Brendan Eich,Frontend Engineer,The web runtime and DX.,San Francisco,brendan@example.com,brendaneich,,,https://brendaneich.com,#f7df1e,system",
    "satoshi-nakamoto,general,Satoshi Nakamoto,Software Engineer,Distributed systems curiosity.,Unknown,satoshi@example.com,,,,,#a855f7,dark",
    "katherine-johnson,data,Katherine Johnson,Data Scientist,Numbers that navigate.,West Virginia,katherine@example.com,kj,,,https://nasa.gov,#06b6d4,system",
    "tim-berners-lee,fullstack,Tim Berners-Lee,Full-Stack Engineer,Hypertext and open web standards.,London,tim@example.com,timbl,https://linkedin.com/in/timbernerslee,https://w3.org,,#0ea5e9,light",
];

const DOMAINS: &[&str] = &[
    "frontend",
    "backend",
    "fullstack",
    "ml",
    "mobile",
    "devops",
    "data",
    "security",
    "game",
    "general",
];

const COLORS: &[&str] = &[
    "#ec4899", "#3b82f6", "#0d9488", "#6366f1", "#f97316", "#ef4444", "#8b5cf6", "#06b6d4",
];

const THEMES: &[&str] = &["system", "light", "dark"];

#[derive(Debug, Clone)]
pub struct SampleCsvOptions {
    /// Number of data rows to write (not counting header).
    pub count: usize,
    /// When true, append a few intentionally invalid rows for error-path testing.
    pub with_breaking: bool,
}

impl Default for SampleCsvOptions {
    fn default() -> Self {
        Self {
            count: DEMO_ROWS.len(),
            with_breaking: false,
        }
    }
}

/// Write a multi-portfolio sample CSV to `path` (streaming, low memory).
pub fn write_sample_csv(path: &Path, opts: &SampleCsvOptions) -> Result<usize> {
    if opts.count == 0 {
        bail!("--count must be at least 1");
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create dir {}", parent.display()))?;
        }
    }

    let file = File::create(path).with_context(|| format!("create {}", path.display()))?;
    let mut w = BufWriter::new(file);
    writeln!(w, "{SAMPLE_CSV_HEADER}")?;

    let mut written = 0usize;
    while written < opts.count {
        if written < DEMO_ROWS.len() {
            writeln!(w, "{}", DEMO_ROWS[written])?;
        } else {
            let i = written + 1;
            let domain = DOMAINS[i % DOMAINS.len()];
            let color = COLORS[i % COLORS.len()];
            let theme = THEMES[i % THEMES.len()];
            let slug = format!("dev-{i:04}");
            let name = format!("Developer {i}");
            let title = format!("{} Engineer", domain_title(domain));
            // slug,domain,name,title,bio,location,email,github,linkedin,website,resume_url,primary,theme
            writeln!(
                w,
                "{slug},{domain},{name},{title},Sample bio for bulk import testing.,Remote,dev{i}@example.com,dev{i},https://linkedin.com/in/dev{i},https://dev{i}.example,,{color},{theme}"
            )?;
        }
        written += 1;
    }

    if opts.with_breaking {
        writeln!(w, "bad-empty-name,frontend,,Engineer,no name,,,,,,,system")?;
        writeln!(
            w,
            "bad-domain,not-a-domain,Bad Domain,Engineer,unknown domain,,,,,,,system"
        )?;
        writeln!(w, ",fullstack,,,empty slug and name will fail,,,,,,,system")?;
    }

    w.flush()?;
    Ok(written)
}

fn domain_title(domain: &str) -> &str {
    match domain {
        "frontend" => "Frontend",
        "backend" => "Backend",
        "fullstack" => "Full-Stack",
        "ml" => "ML",
        "mobile" => "Mobile",
        "devops" => "Platform",
        "data" => "Data",
        "security" => "Security",
        "game" => "Game",
        _ => "Software",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn writes_header_and_demo_rows() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("sample.csv");
        let n = write_sample_csv(
            &path,
            &SampleCsvOptions {
                count: 3,
                with_breaking: false,
            },
        )
        .unwrap();
        assert_eq!(n, 3);
        let text = fs::read_to_string(&path).unwrap();
        let mut lines = text.lines();
        assert_eq!(lines.next().unwrap(), SAMPLE_CSV_HEADER);
        assert!(lines.next().unwrap().contains("ada-lovelace"));
        assert_eq!(lines.count(), 2);
    }

    #[test]
    fn synthesizes_beyond_demo_set() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("large.csv");
        let n = write_sample_csv(
            &path,
            &SampleCsvOptions {
                count: 20,
                with_breaking: true,
            },
        )
        .unwrap();
        assert_eq!(n, 20);
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.contains("Developer"));
        assert!(text.contains("bad-empty-name"));
        assert!(text.lines().count() > 21); // header + 20 + breaking
    }
}
