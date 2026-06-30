mod cli;
mod config;
mod csv_ndjson;
mod domain;
mod prompts;
mod scaffold;
mod slug_util;

use anyhow::{Context, Result};
use clap::Parser;
use console::style;

use cli::{Cli, Commands, CreateArgs, CsvToNdjsonArgs};
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
    }
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
