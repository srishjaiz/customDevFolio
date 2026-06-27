//! Materialize the embedded Next.js template and inject portfolio.json.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rust_embed::Embed;

use crate::config::{sanitize_project_name, PortfolioConfig};

/// Embedded copy of `../template` (built at compile time).
/// Excludes heavy/generated paths so the binary stays lean.
#[derive(Embed)]
#[folder = "../template/"]
#[exclude = "node_modules/**"]
#[exclude = ".next/**"]
#[exclude = "out/**"]
#[exclude = "**/.DS_Store"]
#[exclude = "tsconfig.tsbuildinfo"]
#[exclude = "content/portfolio.json"]
struct TemplateAssets;

#[derive(Debug, Clone)]
pub struct ScaffoldOptions {
    /// Directory that will contain the Next app (created if missing).
    pub output_dir: PathBuf,
    /// npm/package.json name field.
    pub package_name: String,
    pub portfolio: PortfolioConfig,
    /// Overwrite non-empty output directory.
    pub force: bool,
    /// Run `git init` when `git` is on PATH.
    pub git_init: bool,
}

pub fn scaffold(opts: ScaffoldOptions) -> Result<PathBuf> {
    let package_name = sanitize_project_name(&opts.package_name);
    let out = &opts.output_dir;

    prepare_output_dir(out, opts.force)?;

    let entries: Vec<_> = TemplateAssets::iter().collect();
    if entries.is_empty() {
        bail!(
            "embedded template is empty — build the CLI from the monorepo so ../template is present"
        );
    }

    let pb = ProgressBar::new(entries.len() as u64 + 2);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:30.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message("copying template");

    for path in &entries {
        let Some(file) = TemplateAssets::get(path) else {
            continue;
        };
        let dest = out.join(path.as_ref());
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create dir {}", parent.display()))?;
        }

        let data = transform_file(path, file.data.as_ref(), &package_name)?;
        fs::write(&dest, data).with_context(|| format!("write {}", dest.display()))?;
        pb.inc(1);
    }

    // Ensure content/ exists and write generated portfolio.
    let content_dir = out.join("content");
    fs::create_dir_all(&content_dir)?;
    let portfolio_path = content_dir.join("portfolio.json");
    let json = opts
        .portfolio
        .to_pretty_json()
        .context("serialize portfolio.json")?;
    fs::write(&portfolio_path, json)
        .with_context(|| format!("write {}", portfolio_path.display()))?;
    pb.inc(1);
    pb.set_message("portfolio.json");

    // public/images placeholder if missing from embed edge cases
    let images = out.join("public/images");
    if !images.exists() {
        fs::create_dir_all(&images)?;
        fs::write(images.join(".gitkeep"), b"")?;
    }
    pb.inc(1);

    if opts.git_init {
        pb.set_message("git init");
        let _ = std::process::Command::new("git")
            .arg("init")
            .current_dir(out)
            .status();
    }

    pb.finish_with_message("done");
    Ok(out.canonicalize().unwrap_or_else(|_| out.clone()))
}

fn prepare_output_dir(out: &Path, force: bool) -> Result<()> {
    if out.exists() {
        if !out.is_dir() {
            bail!("{} exists and is not a directory", out.display());
        }
        let is_empty = fs::read_dir(out)?.next().is_none();
        if !is_empty && !force {
            bail!(
                "output directory {} is not empty (pass --force to overwrite)",
                out.display()
            );
        }
        if !is_empty && force {
            // Remove contents but keep the directory node.
            for entry in fs::read_dir(out)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(&path)
                        .with_context(|| format!("remove {}", path.display()))?;
                } else {
                    fs::remove_file(&path).with_context(|| format!("remove {}", path.display()))?;
                }
            }
        }
    } else {
        fs::create_dir_all(out).with_context(|| format!("create {}", out.display()))?;
    }
    Ok(())
}

fn transform_file(path: &str, data: &[u8], package_name: &str) -> Result<Vec<u8>> {
    // Only rewrite known text manifests; leave binaries untouched.
    if path == "package.json" {
        let text = std::str::from_utf8(data).context("package.json utf-8")?;
        // Minimal, robust replace of the "name" field.
        let replaced = if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(text) {
            if let Some(obj) = v.as_object_mut() {
                obj.insert(
                    "name".into(),
                    serde_json::Value::String(package_name.to_string()),
                );
            }
            serde_json::to_vec_pretty(&v).unwrap_or_else(|_| data.to_vec())
        } else {
            data.to_vec()
        };
        return Ok(replaced);
    }
    Ok(data.to_vec())
}

/// Resolve output path: `./<name>` when only a name is given.
pub fn resolve_output_dir(name_or_path: &str) -> PathBuf {
    let p = PathBuf::from(name_or_path);
    if p.is_absolute() || name_or_path.contains('/') || name_or_path.contains('\\') {
        p
    } else {
        PathBuf::from(".").join(sanitize_project_name(name_or_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DomainId, ThemeMode};
    use crate::domain::CreateAnswers;

    #[test]
    fn scaffold_writes_portfolio_and_package_name() {
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("demo-site");
        let portfolio = CreateAnswers {
            project_name: "demo-site".into(),
            domain: DomainId::Ml,
            display_name: "Sam".into(),
            title: None,
            bio: None,
            location: None,
            email: Some("sam@example.com".into()),
            github: Some("sam".into()),
            linkedin: None,
            website: None,
            resume_url: None,
            primary_color: None,
            theme_mode: ThemeMode::System,
            include_sample_content: true,
        }
        .into_portfolio();

        scaffold(ScaffoldOptions {
            output_dir: out.clone(),
            package_name: "demo-site".into(),
            portfolio,
            force: false,
            git_init: false,
        })
        .unwrap();

        assert!(out.join("package.json").is_file());
        assert!(out.join("app/page.tsx").is_file());
        assert!(out.join("lib/domains.ts").is_file());
        let pkg = fs::read_to_string(out.join("package.json")).unwrap();
        assert!(pkg.contains("\"demo-site\""));
        let port = fs::read_to_string(out.join("content/portfolio.json")).unwrap();
        assert!(port.contains("\"ml\"") || port.contains("\"domain\": \"ml\""));
        // Embedded template should not include node_modules
        assert!(!out.join("node_modules").exists());
    }
}
