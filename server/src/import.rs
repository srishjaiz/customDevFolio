//! Stream NDJSON portfolio records into Postgres (Phase 3).
//! Reads line-by-line — never loads the full NDJSON file into memory.

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::error::{RepoError, RepoResult};
use crate::models::{ImportJobStatus, NewPortfolio};
use crate::repos::{ImportJobRepo, PortfolioRepo};
use crate::slug::is_valid_slug;
use crate::PgPool;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportStats {
    pub total_rows: u64,
    pub succeeded: u64,
    pub failed: u64,
}

/// Stream `ndjson_path` and upsert portfolios under `account_id`.
///
/// Optional `errors_path` receives `{"line", "error"}` NDJSON for failed lines.
/// When `job_id` is set, updates `import_jobs` status and counters.
pub async fn import_ndjson_file(
    pool: &PgPool,
    account_id: Uuid,
    ndjson_path: &Path,
    errors_path: Option<&Path>,
    continue_on_error: bool,
    job_id: Option<Uuid>,
) -> RepoResult<ImportStats> {
    let jobs = ImportJobRepo::new(pool);
    let portfolios = PortfolioRepo::new(pool);

    if let Some(id) = job_id {
        let _ = jobs
            .update_status(id, ImportJobStatus::Importing, None)
            .await;
        let _ = jobs
            .update_paths_and_counts(
                id,
                Some(&ndjson_path.display().to_string()),
                errors_path.map(|p| p.display().to_string()).as_deref(),
                0,
                0,
                0,
            )
            .await;
    }

    let file = File::open(ndjson_path).map_err(RepoError::Io)?;
    let reader = BufReader::new(file);

    let mut err_writer = if let Some(path) = errors_path {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(RepoError::Io)?;
            }
        }
        let f = File::create(path).map_err(RepoError::Io)?;
        Some(BufWriter::new(f))
    } else {
        None
    };

    let mut stats = ImportStats::default();

    for (idx, line_res) in reader.lines().enumerate() {
        let line_no = (idx + 1) as u64;
        let line = match line_res {
            Ok(l) => l,
            Err(e) => {
                stats.total_rows += 1;
                stats.failed += 1;
                write_err(&mut err_writer, line_no, &e.to_string())?;
                if !continue_on_error {
                    fail_job(&jobs, job_id, &stats, &e.to_string()).await?;
                    return Err(RepoError::Invalid(format!("read line {line_no}: {e}")));
                }
                continue;
            }
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        stats.total_rows += 1;

        match parse_and_upsert(&portfolios, account_id, line).await {
            Ok(()) => stats.succeeded += 1,
            Err(e) => {
                stats.failed += 1;
                write_err(&mut err_writer, line_no, &e.to_string())?;
                if !continue_on_error {
                    fail_job(&jobs, job_id, &stats, &e.to_string()).await?;
                    return Err(e);
                }
            }
        }
    }

    if let Some(ref mut w) = err_writer {
        w.flush().map_err(RepoError::Io)?;
    }

    if let Some(id) = job_id {
        let _ = jobs
            .update_paths_and_counts(
                id,
                Some(&ndjson_path.display().to_string()),
                errors_path.map(|p| p.display().to_string()).as_deref(),
                stats.total_rows as i64,
                stats.succeeded as i64,
                stats.failed as i64,
            )
            .await;
        let status = if stats.failed > 0 && stats.succeeded == 0 {
            ImportJobStatus::Failed
        } else {
            ImportJobStatus::Completed
        };
        let msg = if stats.failed > 0 {
            Some(format!("{} row(s) failed", stats.failed))
        } else {
            None
        };
        let _ = jobs.update_status(id, status, msg.as_deref()).await;
    }

    Ok(stats)
}

async fn parse_and_upsert(
    portfolios: &PortfolioRepo<'_>,
    account_id: Uuid,
    line: &str,
) -> RepoResult<()> {
    let value: JsonValue =
        serde_json::from_str(line).map_err(|e| RepoError::Invalid(format!("invalid JSON: {e}")))?;

    let slug = value
        .get("slug")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RepoError::Invalid("missing slug".into()))?
        .to_string();
    if !is_valid_slug(&slug) {
        return Err(RepoError::Invalid(format!("invalid slug '{slug}'")));
    }

    let domain = value
        .get("domain")
        .and_then(|v| v.as_str())
        .unwrap_or("fullstack")
        .to_string();

    let person_name = value
        .get("person")
        .and_then(|p| p.get("name"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| RepoError::Invalid("missing person.name".into()))?
        .to_string();

    // Store full document minus top-level slug (config is PortfolioConfig-shaped).
    let mut config = value;
    if let Some(obj) = config.as_object_mut() {
        obj.remove("slug");
    }

    portfolios
        .upsert(&NewPortfolio {
            account_id,
            slug,
            domain,
            person_name,
            config,
        })
        .await?;
    Ok(())
}

fn write_err(
    err_writer: &mut Option<BufWriter<File>>,
    line_no: u64,
    message: &str,
) -> RepoResult<()> {
    if let Some(w) = err_writer {
        let obj = serde_json::json!({ "line": line_no, "error": message });
        serde_json::to_writer(&mut *w, &obj).map_err(|e| RepoError::Invalid(e.to_string()))?;
        w.write_all(b"\n").map_err(RepoError::Io)?;
    }
    Ok(())
}

async fn fail_job(
    jobs: &ImportJobRepo<'_>,
    job_id: Option<Uuid>,
    stats: &ImportStats,
    message: &str,
) -> RepoResult<()> {
    if let Some(id) = job_id {
        let _ = jobs
            .update_paths_and_counts(
                id,
                None,
                None,
                stats.total_rows as i64,
                stats.succeeded as i64,
                stats.failed as i64,
            )
            .await;
        let _ = jobs
            .update_status(id, ImportJobStatus::Failed, Some(message))
            .await;
    }
    Ok(())
}
