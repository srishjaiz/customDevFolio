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
