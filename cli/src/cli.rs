use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use crate::config::{DomainId, ThemeMode};

#[derive(Debug, Parser)]
#[command(
    name = "customfolio",
    about = "Scaffold domain-aware Next.js developer portfolios",
    version,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new portfolio app from the embedded Next.js template
    Create(Box<CreateArgs>),
    /// List supported SWE domains and defaults
    Domains,
    /// Stream a portfolio CSV to NDJSON on disk (large-file safe)
    CsvToNdjson(CsvToNdjsonArgs),
}

#[derive(Debug, clap::Args)]
pub struct CsvToNdjsonArgs {
    /// Input CSV path (header row required)
    pub input: PathBuf,

    /// Output NDJSON path (one portfolio JSON object per line)
    #[arg(short = 'o', long)]
    pub output: PathBuf,

    /// Include domain sample experience/projects (default: minimal placeholders only)
    #[arg(long)]
    pub sample: bool,

    /// Continue after row errors; write remaining good rows
    #[arg(long)]
    pub continue_on_error: bool,

    /// Write per-row errors as NDJSON (`{"line", "error"}`)
    #[arg(long)]
    pub errors: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ThemeModeArg {
    System,
    Light,
    Dark,
}

impl From<ThemeModeArg> for ThemeMode {
    fn from(v: ThemeModeArg) -> Self {
        match v {
            ThemeModeArg::System => ThemeMode::System,
            ThemeModeArg::Light => ThemeMode::Light,
            ThemeModeArg::Dark => ThemeMode::Dark,
        }
    }
}

#[derive(Debug, clap::Args)]
pub struct CreateArgs {
    /// Project directory name or path (default: my-portfolio)
    #[arg(default_value = "my-portfolio")]
    pub name: String,

    /// Explicit output directory (defaults to ./<name>)
    #[arg(long, short = 'o')]
    pub output: Option<PathBuf>,

    /// SWE domain preset
    #[arg(long, short = 'd', value_parser = parse_domain)]
    pub domain: Option<DomainId>,

    /// Display name on the portfolio
    #[arg(long)]
    pub full_name: Option<String>,

    /// Role title (defaults from domain)
    #[arg(long)]
    pub title: Option<String>,

    /// Short bio
    #[arg(long)]
    pub bio: Option<String>,

    /// Location string
    #[arg(long)]
    pub location: Option<String>,

    /// Contact email
    #[arg(long)]
    pub email: Option<String>,

    /// GitHub username or URL
    #[arg(long)]
    pub github: Option<String>,

    /// LinkedIn profile URL
    #[arg(long)]
    pub linkedin: Option<String>,

    /// Personal site URL
    #[arg(long)]
    pub website: Option<String>,

    /// Resume link
    #[arg(long)]
    pub resume_url: Option<String>,

    /// Primary brand color (CSS), e.g. #6366f1
    #[arg(long)]
    pub primary: Option<String>,

    /// Initial color mode
    #[arg(long, value_enum, default_value_t = ThemeModeArg::System)]
    pub theme: ThemeModeArg,

    /// Skip interactive prompts; use flags + domain defaults
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// Overwrite non-empty output directory
    #[arg(long, short = 'f')]
    pub force: bool,

    /// Run git init in the output directory
    #[arg(long)]
    pub git: bool,

    /// Skip sample experience/projects (minimal placeholders only)
    #[arg(long)]
    pub minimal: bool,
}

fn parse_domain(s: &str) -> Result<DomainId, String> {
    s.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn theme_mode_arg_converts() {
        assert_eq!(ThemeMode::from(ThemeModeArg::System), ThemeMode::System);
        assert_eq!(ThemeMode::from(ThemeModeArg::Light), ThemeMode::Light);
        assert_eq!(ThemeMode::from(ThemeModeArg::Dark), ThemeMode::Dark);
    }

    #[test]
    fn parse_domain_accepts_aliases() {
        assert_eq!(parse_domain("fe").unwrap(), DomainId::Frontend);
        assert_eq!(parse_domain("devops").unwrap(), DomainId::Devops);
        assert!(parse_domain("nope").is_err());
    }

    #[test]
    fn cli_parses_create_defaults() {
        let cli = Cli::try_parse_from(["customfolio", "create"]).unwrap();
        match cli.command {
            Commands::Create(args) => {
                assert_eq!(args.name, "my-portfolio");
                assert!(!args.yes);
                assert!(!args.force);
                assert!(!args.git);
                assert!(!args.minimal);
                assert!(args.domain.is_none());
                assert!(matches!(args.theme, ThemeModeArg::System));
            }
            Commands::Domains | Commands::CsvToNdjson(_) => panic!("expected create"),
        }
    }

    #[test]
    fn cli_parses_create_flags() {
        let cli = Cli::try_parse_from([
            "customfolio",
            "create",
            "site",
            "-o",
            "/tmp/out",
            "-d",
            "ml",
            "--full-name",
            "Sam",
            "--title",
            "ML Eng",
            "--bio",
            "hello",
            "--location",
            "NYC",
            "--email",
            "s@x.test",
            "--github",
            "sam",
            "--linkedin",
            "https://linkedin.com/in/sam",
            "--website",
            "https://sam.test",
            "--resume-url",
            "https://sam.test/cv",
            "--primary",
            "#0d9488",
            "--theme",
            "dark",
            "-y",
            "-f",
            "--git",
            "--minimal",
        ])
        .unwrap();
        match cli.command {
            Commands::Create(args) => {
                assert_eq!(args.name, "site");
                assert_eq!(
                    args.output.as_deref(),
                    Some(std::path::Path::new("/tmp/out"))
                );
                assert_eq!(args.domain, Some(DomainId::Ml));
                assert_eq!(args.full_name.as_deref(), Some("Sam"));
                assert_eq!(args.title.as_deref(), Some("ML Eng"));
                assert_eq!(args.bio.as_deref(), Some("hello"));
                assert_eq!(args.location.as_deref(), Some("NYC"));
                assert_eq!(args.email.as_deref(), Some("s@x.test"));
                assert_eq!(args.github.as_deref(), Some("sam"));
                assert_eq!(
                    args.linkedin.as_deref(),
                    Some("https://linkedin.com/in/sam")
                );
                assert_eq!(args.website.as_deref(), Some("https://sam.test"));
                assert_eq!(args.resume_url.as_deref(), Some("https://sam.test/cv"));
                assert_eq!(args.primary.as_deref(), Some("#0d9488"));
                assert!(matches!(args.theme, ThemeModeArg::Dark));
                assert!(args.yes);
                assert!(args.force);
                assert!(args.git);
                assert!(args.minimal);
            }
            Commands::Domains | Commands::CsvToNdjson(_) => panic!("expected create"),
        }
    }

    #[test]
    fn cli_parses_domains_subcommand() {
        let cli = Cli::try_parse_from(["customfolio", "domains"]).unwrap();
        assert!(matches!(cli.command, Commands::Domains));
    }

    #[test]
    fn cli_parses_csv_to_ndjson() {
        let cli = Cli::try_parse_from([
            "customfolio",
            "csv-to-ndjson",
            "in.csv",
            "-o",
            "out.ndjson",
            "--sample",
            "--continue-on-error",
            "--errors",
            "err.ndjson",
        ])
        .unwrap();
        match cli.command {
            Commands::CsvToNdjson(args) => {
                assert_eq!(args.input, PathBuf::from("in.csv"));
                assert_eq!(args.output, PathBuf::from("out.ndjson"));
                assert!(args.sample);
                assert!(args.continue_on_error);
                assert_eq!(
                    args.errors.as_deref(),
                    Some(std::path::Path::new("err.ndjson"))
                );
            }
            _ => panic!("expected csv-to-ndjson"),
        }
    }

    #[test]
    fn cli_rejects_unknown_domain_flag() {
        let err = Cli::try_parse_from(["customfolio", "create", "--domain", "nope"]).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("unknown domain") || msg.contains("invalid value"));
    }
}
