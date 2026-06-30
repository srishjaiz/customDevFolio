mod cli;
mod config;
mod csv_ndjson;
mod domain;
mod prompts;
mod scaffold;
mod slug_util;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use console::style;

use cli::{Cli, Commands, CreateArgs, CsvToNdjsonArgs, ImportDbArgs};
use config::{sanitize_project_name, DomainId, ThemeMode};
use csv_ndjson::{csv_to_ndjson, ConvertOptions};
use domain::CreateAnswers;
use prompts::{apply_defaults, print_domains, run_wizard};
use scaffold::{resolve_output_dir, scaffold, ScaffoldOptions};

fn main() {
    if let Err(err) = run() {
        eprintln!("{} {:#}", style("error:").red().bold(), err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Domains => {
            print_domains();
            Ok(())
        }
        Commands::Create(args) => cmd_create(*args),
        Commands::CsvToNdjson(args) => cmd_csv_to_ndjson(args),
        Commands::ImportDb(args) => cmd_import_db(args),
    }
}

fn cmd_import_db(args: ImportDbArgs) -> Result<()> {
    let rt = tokio::runtime::Runtime::new().context("tokio runtime")?;
    rt.block_on(import_db_async(args))
}

async fn import_db_async(args: ImportDbArgs) -> Result<()> {
    use customfolio_server::{
        connect, import_ndjson_file, migrate, AccountRepo, ImportJobRepo, ImportJobStatus,
        NewImportJob,
    };
    use uuid::Uuid;

    let account_id = Uuid::parse_str(&args.account_id).context("parse --account-id")?;
    let user_id = Uuid::parse_str(&args.user_id).context("parse --user-id")?;

    println!(
        "\n{} Import DB from {} …\n",
        style("→").green().bold(),
        style(args.csv.display()).bold()
    );

    let pool = connect(&args.database_url)
        .await
        .context("connect to postgres")?;
    migrate(&pool).await.context("run migrations")?;

    // Ensure account exists
    AccountRepo::new(&pool)
        .get_by_id(account_id)
        .await
        .context("account not found — create user/account first")?;

    let job = ImportJobRepo::new(&pool)
        .create(&NewImportJob {
            account_id,
            user_id,
            source_filename: args
                .csv
                .file_name()
                .map(|s| s.to_string_lossy().into_owned()),
            csv_path: Some(args.csv.display().to_string()),
        })
        .await
        .context("create import job")?;

    let work_dir = args
        .work_dir
        .unwrap_or_else(|| PathBuf::from(format!("data/imports/{}", job.id)));
    std::fs::create_dir_all(&work_dir)
        .with_context(|| format!("create work dir {}", work_dir.display()))?;

    let ndjson_path = work_dir.join("data.ndjson");
    let errors_path = work_dir.join("errors.ndjson");

    ImportJobRepo::new(&pool)
        .update_status(job.id, ImportJobStatus::Converting, None)
        .await
        .ok();

    println!(
        "  {} CSV → NDJSON ({})",
        style("1.").dim(),
        ndjson_path.display()
    );
    let conv = csv_to_ndjson(
        &args.csv,
        &ndjson_path,
        &ConvertOptions {
            include_sample_content: args.sample,
            continue_on_error: args.continue_on_error,
            errors_path: Some(errors_path.clone()),
        },
    )
    .context("csv to ndjson")?;

    println!(
        "  {} convert: {} ok, {} failed",
        style("·").dim(),
        conv.succeeded,
        conv.failed
    );

    println!("  {} NDJSON → Postgres", style("2.").dim());
    let stats = import_ndjson_file(
        &pool,
        account_id,
        &ndjson_path,
        Some(&errors_path),
        args.continue_on_error,
        Some(job.id),
    )
    .await
    .map_err(|e| anyhow::anyhow!("{e}"))?;

    println!();
    println!("{} Import complete", style("✓").green().bold());
    println!("  job:       {}", job.id);
    println!("  rows:      {}", stats.total_rows);
    println!("  succeeded: {}", stats.succeeded);
    println!("  failed:    {}", stats.failed);
    println!("  work dir:  {}", work_dir.display());
    Ok(())
}

fn cmd_csv_to_ndjson(args: CsvToNdjsonArgs) -> Result<()> {
    println!(
        "\n{} Streaming {} → {} …\n",
        style("→").green().bold(),
        style(args.input.display()).bold(),
        style(args.output.display()).bold()
    );

    let stats = csv_to_ndjson(
        &args.input,
        &args.output,
        &ConvertOptions {
            include_sample_content: args.sample,
            continue_on_error: args.continue_on_error,
            errors_path: args.errors.clone(),
        },
    )
    .with_context(|| {
        format!(
            "convert {} to {}",
            args.input.display(),
            args.output.display()
        )
    })?;

    println!("{} Conversion complete", style("✓").green().bold());
    println!();
    println!("  rows:      {}", stats.total_rows);
    println!("  succeeded: {}", stats.succeeded);
    println!("  failed:    {}", stats.failed);
    println!();
    println!("  {}", style(args.output.display()).cyan());
    if let Some(ref err) = args.errors {
        if stats.failed > 0 {
            println!("  errors: {}", style(err.display()).yellow());
        }
    }
    println!();
    println!("  NDJSON is an on-disk intermediate (Phase 2). Import into Postgres is Phase 3.");
    Ok(())
}

fn cmd_create(args: CreateArgs) -> Result<()> {
    let project_name = sanitize_project_name(&args.name);

    let partial = CreateAnswers {
        project_name: project_name.clone(),
        domain: args.domain.unwrap_or(DomainId::Fullstack),
        display_name: args.full_name.clone().unwrap_or_else(|| "Developer".into()),
        title: args.title.clone(),
        bio: args.bio.clone(),
        location: args.location.clone(),
        email: args.email.clone(),
        github: args.github.clone(),
        linkedin: args.linkedin.clone(),
        website: args.website.clone(),
        resume_url: args.resume_url.clone(),
        primary_color: args.primary.clone(),
        theme_mode: ThemeMode::from(args.theme),
        include_sample_content: !args.minimal,
    };

    // Interactive unless --yes or non-TTY stdin (CI).
    let interactive = !args.yes && console::user_attended();
    let answers = if interactive {
        run_wizard(partial)?
    } else {
        apply_defaults(partial)
    };

    let package_name = answers.project_name.clone();
    // Honor explicit --output; otherwise derive from final project name (wizard may change it).
    let output_dir = args
        .output
        .clone()
        .unwrap_or_else(|| resolve_output_dir(&package_name));
    let portfolio = answers.into_portfolio();

    println!(
        "\n{} Generating {} ({}) …\n",
        style("→").green().bold(),
        style(output_dir.display()).bold(),
        portfolio.domain
    );

    let final_path = scaffold(ScaffoldOptions {
        output_dir: output_dir.clone(),
        package_name: package_name.clone(),
        portfolio,
        force: args.force,
        git_init: args.git,
    })
    .with_context(|| format!("scaffold into {}", output_dir.display()))?;

    println!();
    println!("{} Portfolio app ready", style("✓").green().bold());
    println!();
    println!("  {}", style(final_path.display()).cyan());
    println!();
    println!("{}", style("Next steps").bold());
    println!("  cd {}", shell_cd_path(&final_path));
    println!("  pnpm install    # or npm install / yarn");
    println!("  pnpm dev");
    println!();
    println!(
        "  Edit {} to customize content.",
        style("content/portfolio.json").yellow()
    );
    println!("  Domain presets & section flags are documented in the template README.");

    let _ = package_name;
    Ok(())
}

fn shell_cd_path(path: &std::path::Path) -> String {
    let s = path.display().to_string();
    if s.contains(' ') {
        format!("\"{s}\"")
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn shell_cd_path_quotes_spaces() {
        assert_eq!(shell_cd_path(Path::new("/tmp/my app")), "\"/tmp/my app\"");
        assert_eq!(shell_cd_path(Path::new("/tmp/app")), "/tmp/app");
    }
}
