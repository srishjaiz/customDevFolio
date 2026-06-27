mod cli;
mod config;
mod domain;
mod prompts;
mod scaffold;

use anyhow::{Context, Result};
use clap::Parser;
use console::style;

use cli::{Cli, Commands, CreateArgs};
use config::{sanitize_project_name, DomainId, ThemeMode};
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
    }
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
