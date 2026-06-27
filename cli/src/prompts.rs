//! Interactive create wizard (dialoguer). Skipped when `--yes` is set.

use anyhow::{Context, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input, Select};

use crate::config::{sanitize_project_name, DomainId, ThemeMode};
use crate::domain::{list_profiles, profile, CreateAnswers};

pub fn run_wizard(partial: CreateAnswers) -> Result<CreateAnswers> {
    let theme = ColorfulTheme::default();
    println!(
        "\n{}  Domain-aware Next.js portfolio scaffold\n",
        style("customFolio").cyan().bold()
    );

    let project_name: String = Input::with_theme(&theme)
        .with_prompt("Project directory / package name")
        .default(partial.project_name.clone())
        .interact_text()
        .context("project name")?;
    let project_name = sanitize_project_name(&project_name);

    let profiles = list_profiles();
    let labels: Vec<String> = profiles
        .iter()
        .map(|p| format!("{} — {}", p.label, p.description))
        .collect();
    let default_idx = profiles
        .iter()
        .position(|p| p.id == partial.domain)
        .unwrap_or(2);
    let domain_idx = FuzzySelect::with_theme(&theme)
        .with_prompt("Developer domain")
        .items(&labels)
        .default(default_idx)
        .interact()
        .context("domain")?;
    let domain = profiles[domain_idx].id;
    let dom = profile(domain);

    let display_name: String = Input::with_theme(&theme)
        .with_prompt("Your display name")
        .default(partial.display_name.clone())
        .interact_text()?;

    let title: String = Input::with_theme(&theme)
        .with_prompt("Role title")
        .default(
            partial
                .title
                .clone()
                .unwrap_or_else(|| dom.default_title.to_string()),
        )
        .interact_text()?;

    let bio: String = Input::with_theme(&theme)
        .with_prompt("Short bio (one or two sentences)")
        .allow_empty(true)
        .default(partial.bio.clone().unwrap_or_default())
        .interact_text()?;

    let location: String = Input::with_theme(&theme)
        .with_prompt("Location (optional)")
        .allow_empty(true)
        .default(partial.location.clone().unwrap_or_default())
        .interact_text()?;

    let email: String = Input::with_theme(&theme)
        .with_prompt("Email (optional)")
        .allow_empty(true)
        .default(partial.email.clone().unwrap_or_default())
        .interact_text()?;

    let github: String = Input::with_theme(&theme)
        .with_prompt("GitHub username or URL (optional)")
        .allow_empty(true)
        .default(partial.github.clone().unwrap_or_default())
        .interact_text()?;

    let linkedin: String = Input::with_theme(&theme)
        .with_prompt("LinkedIn URL (optional)")
        .allow_empty(true)
        .default(partial.linkedin.clone().unwrap_or_default())
        .interact_text()?;

    let website: String = Input::with_theme(&theme)
        .with_prompt("Website / portfolio URL (optional)")
        .allow_empty(true)
        .default(partial.website.clone().unwrap_or_default())
        .interact_text()?;

    let resume_url: String = Input::with_theme(&theme)
        .with_prompt("Resume URL (optional)")
        .allow_empty(true)
        .default(partial.resume_url.clone().unwrap_or_default())
        .interact_text()?;

    let primary_color: String = Input::with_theme(&theme)
        .with_prompt("Primary brand color (CSS hex)")
        .default(
            partial
                .primary_color
                .clone()
                .unwrap_or_else(|| dom.default_primary.to_string()),
        )
        .interact_text()?;

    let modes = ["system", "light", "dark"];
    let mode_default = match partial.theme_mode {
        ThemeMode::System => 0,
        ThemeMode::Light => 1,
        ThemeMode::Dark => 2,
    };
    let mode_idx = Select::with_theme(&theme)
        .with_prompt("Initial theme mode")
        .items(&modes)
        .default(mode_default)
        .interact()?;
    let theme_mode: ThemeMode = modes[mode_idx].parse().unwrap();

    let include_sample_content = Confirm::with_theme(&theme)
        .with_prompt("Include sample experience / projects for a full preview?")
        .default(partial.include_sample_content)
        .interact()?;

    println!();
    println!("{}", style("Summary").bold());
    println!("  Project : {project_name}");
    println!("  Domain  : {} ({})", dom.label, domain);
    println!("  Name    : {display_name}");
    println!("  Title   : {title}");
    println!("  Theme   : {primary_color} / {}", theme_mode.as_str());
    let ok = Confirm::with_theme(&theme)
        .with_prompt("Generate portfolio app?")
        .default(true)
        .interact()?;
    if !ok {
        anyhow::bail!("cancelled");
    }

    Ok(CreateAnswers {
        project_name,
        domain,
        display_name,
        title: Some(title).filter(|s| !s.is_empty()),
        bio: Some(bio).filter(|s| !s.is_empty()),
        location: Some(location).filter(|s| !s.is_empty()),
        email: Some(email).filter(|s| !s.is_empty()),
        github: Some(github).filter(|s| !s.is_empty()),
        linkedin: Some(linkedin).filter(|s| !s.is_empty()),
        website: Some(website).filter(|s| !s.is_empty()),
        resume_url: Some(resume_url).filter(|s| !s.is_empty()),
        primary_color: Some(primary_color).filter(|s| !s.is_empty()),
        theme_mode,
        include_sample_content,
    })
}

/// Non-interactive path: fill gaps with domain defaults.
pub fn apply_defaults(mut answers: CreateAnswers) -> CreateAnswers {
    let p = profile(answers.domain);
    if answers.display_name.trim().is_empty() {
        answers.display_name = "Developer".into();
    }
    if answers
        .title
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
    {
        answers.title = Some(p.default_title.into());
    }
    if answers
        .primary_color
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
    {
        answers.primary_color = Some(p.default_primary.into());
    }
    answers.project_name = sanitize_project_name(&answers.project_name);
    answers
}

pub fn print_domains() {
    println!("{}", style("Supported domains").bold());
    println!();
    for p in list_profiles() {
        println!(
            "  {}  {}",
            style(format!("{:10}", p.id.as_str())).cyan(),
            p.label
        );
        println!("             {}", style(p.description).dim());
    }
    println!();
    println!(
        "Use: {} {}",
        style("customfolio create my-site --domain").dim(),
        style("<id> --yes").dim()
    );
}

// silence unused import in non-test if DomainId only used in docs — keep for API symmetry
#[allow(dead_code)]
fn _domain_id_export() -> DomainId {
    DomainId::General
}
