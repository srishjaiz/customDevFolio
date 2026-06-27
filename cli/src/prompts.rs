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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ThemeMode;
    use crate::domain::{profile, CreateAnswers};

    fn partial() -> CreateAnswers {
        CreateAnswers {
            project_name: "My Site".into(),
            domain: DomainId::Frontend,
            display_name: "".into(),
            title: None,
            bio: None,
            location: None,
            email: None,
            github: None,
            linkedin: None,
            website: None,
            resume_url: None,
            primary_color: None,
            theme_mode: ThemeMode::Light,
            include_sample_content: true,
        }
    }

    #[test]
    fn apply_defaults_fills_gaps() {
        let out = apply_defaults(partial());
        assert_eq!(out.project_name, "my-site");
        assert_eq!(out.display_name, "Developer");
        assert_eq!(
            out.title.as_deref(),
            Some(profile(DomainId::Frontend).default_title)
        );
        assert_eq!(
            out.primary_color.as_deref(),
            Some(profile(DomainId::Frontend).default_primary)
        );
        assert_eq!(out.theme_mode, ThemeMode::Light);
    }

    #[test]
    fn apply_defaults_preserves_set_fields() {
        let mut a = partial();
        a.display_name = "Ada".into();
        a.title = Some("Lead".into());
        a.primary_color = Some("#000000".into());
        a.project_name = "already-ok".into();
        let out = apply_defaults(a);
        assert_eq!(out.display_name, "Ada");
        assert_eq!(out.title.as_deref(), Some("Lead"));
        assert_eq!(out.primary_color.as_deref(), Some("#000000"));
        assert_eq!(out.project_name, "already-ok");
    }

    #[test]
    fn apply_defaults_treats_whitespace_title_as_empty() {
        let mut a = partial();
        a.display_name = "  ".into();
        a.title = Some("   ".into());
        a.primary_color = Some("".into());
        a.domain = DomainId::Ml;
        let out = apply_defaults(a);
        assert_eq!(out.display_name, "Developer");
        assert_eq!(
            out.title.as_deref(),
            Some(profile(DomainId::Ml).default_title)
        );
        assert_eq!(
            out.primary_color.as_deref(),
            Some(profile(DomainId::Ml).default_primary)
        );
    }

    #[test]
    fn print_domains_runs_without_panic() {
        // Smoke: exercises formatting path for all profiles.
        print_domains();
    }
}
