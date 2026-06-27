//! Portfolio configuration mirroring `template/lib/types.ts` / `content/portfolio.json`.

use serde::{Deserialize, Serialize};

/// Software-engineering domains supported by presets and the Next template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DomainId {
    Frontend,
    Backend,
    Fullstack,
    Ml,
    Mobile,
    Devops,
    Data,
    Security,
    Game,
    General,
}

impl DomainId {
    pub const ALL: [DomainId; 10] = [
        DomainId::Frontend,
        DomainId::Backend,
        DomainId::Fullstack,
        DomainId::Ml,
        DomainId::Mobile,
        DomainId::Devops,
        DomainId::Data,
        DomainId::Security,
        DomainId::Game,
        DomainId::General,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            DomainId::Frontend => "frontend",
            DomainId::Backend => "backend",
            DomainId::Fullstack => "fullstack",
            DomainId::Ml => "ml",
            DomainId::Mobile => "mobile",
            DomainId::Devops => "devops",
            DomainId::Data => "data",
            DomainId::Security => "security",
            DomainId::Game => "game",
            DomainId::General => "general",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "frontend" | "fe" => Some(DomainId::Frontend),
            "backend" | "be" => Some(DomainId::Backend),
            "fullstack" | "full-stack" | "full_stack" => Some(DomainId::Fullstack),
            "ml" | "ai" | "machine-learning" => Some(DomainId::Ml),
            "mobile" => Some(DomainId::Mobile),
            "devops" | "sre" => Some(DomainId::Devops),
            "data" => Some(DomainId::Data),
            "security" | "sec" => Some(DomainId::Security),
            "game" | "gamedev" => Some(DomainId::Game),
            "general" | "swe" => Some(DomainId::General),
            _ => None,
        }
    }
}

impl std::fmt::Display for DomainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for DomainId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DomainId::parse(s).ok_or_else(|| {
            format!(
                "unknown domain '{s}'. Expected one of: {}",
                DomainId::ALL
                    .iter()
                    .map(|d| d.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    System,
    Light,
    Dark,
}

impl ThemeMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ThemeMode::System => "system",
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }
}

impl std::str::FromStr for ThemeMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "system" => Ok(ThemeMode::System),
            "light" => Ok(ThemeMode::Light),
            "dark" => Ok(ThemeMode::Dark),
            _ => Err(format!("unknown theme mode '{s}' (system|light|dark)")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionsConfig {
    pub skills: bool,
    pub experience: bool,
    pub projects: bool,
    pub education: bool,
    pub achievements: bool,
    pub blog: bool,
    pub contact: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dribbble: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonInfo {
    pub name: String,
    pub title: String,
    pub bio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Greeting {
    pub headline: String,
    pub subheadline: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta_href: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillItem {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillGroup {
    pub name: String,
    pub items: Vec<SkillItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillsSection {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    pub groups: Vec<SkillGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceItem {
    pub role: String,
    pub company: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tech: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EducationItem {
    pub school: String,
    pub degree: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectLink {
    pub label: String,
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectItem {
    pub title: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "demoUrl", skip_serializing_if = "Option::is_none")]
    pub demo_url: Option<String>,
    #[serde(rename = "repoUrl", skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<ProjectLink>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub featured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementItem {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogPost {
    pub title: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogConfig {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub posts: Vec<BlogPost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactInfo {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "bookingUrl", skip_serializing_if = "Option::is_none")]
    pub booking_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeConfig {
    pub primary: String,
    pub mode: ThemeMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioMeta {
    pub title: String,
    pub description: String,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
}

/// Single source of truth written to `content/portfolio.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioConfig {
    pub meta: PortfolioMeta,
    pub domain: DomainId,
    pub person: PersonInfo,
    pub social: SocialLinks,
    pub greeting: Greeting,
    pub skills: SkillsSection,
    pub experience: Vec<ExperienceItem>,
    pub education: Vec<EducationItem>,
    pub projects: Vec<ProjectItem>,
    pub achievements: Vec<AchievementItem>,
    pub blog: BlogConfig,
    pub contact: ContactInfo,
    pub theme: ThemeConfig,
    pub sections: SectionsConfig,
}

impl PortfolioConfig {
    pub fn to_pretty_json(&self) -> Result<String, serde_json::Error> {
        let mut s = serde_json::to_string_pretty(self)?;
        s.push('\n');
        Ok(s)
    }
}

/// Sanitize a project directory / npm package name.
pub fn sanitize_project_name(raw: &str) -> String {
    let lower = raw.trim().to_ascii_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut prev_dash = false;
    for ch in lower.chars() {
        let ok = ch.is_ascii_alphanumeric() || ch == '-' || ch == '_';
        if ok {
            out.push(if ch == '_' { '-' } else { ch });
            prev_dash = ch == '-' || ch == '_';
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "my-portfolio".into()
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn sanitize_names() {
        assert_eq!(sanitize_project_name("My Portfolio"), "my-portfolio");
        assert_eq!(sanitize_project_name("  "), "my-portfolio");
        assert_eq!(sanitize_project_name("Ada_Lovelace"), "ada-lovelace");
        assert_eq!(sanitize_project_name("---"), "my-portfolio");
        assert_eq!(sanitize_project_name("Foo!!!Bar"), "foo-bar");
        assert_eq!(sanitize_project_name("a__b"), "a--b");
        assert_eq!(sanitize_project_name("Already-Ok"), "already-ok");
        assert_eq!(sanitize_project_name("  spaced  name  "), "spaced-name");
    }

    #[test]
    fn domain_roundtrip() {
        for d in DomainId::ALL {
            assert_eq!(DomainId::parse(d.as_str()), Some(d));
            assert_eq!(d.to_string(), d.as_str());
            assert_eq!(DomainId::from_str(d.as_str()).unwrap(), d);
        }
        assert_eq!(DomainId::ALL.len(), 10);
    }

    #[test]
    fn domain_aliases() {
        assert_eq!(DomainId::parse("fe"), Some(DomainId::Frontend));
        assert_eq!(DomainId::parse("BE"), Some(DomainId::Backend));
        assert_eq!(DomainId::parse("full-stack"), Some(DomainId::Fullstack));
        assert_eq!(DomainId::parse("full_stack"), Some(DomainId::Fullstack));
        assert_eq!(DomainId::parse("ai"), Some(DomainId::Ml));
        assert_eq!(DomainId::parse("machine-learning"), Some(DomainId::Ml));
        assert_eq!(DomainId::parse("sre"), Some(DomainId::Devops));
        assert_eq!(DomainId::parse("sec"), Some(DomainId::Security));
        assert_eq!(DomainId::parse("gamedev"), Some(DomainId::Game));
        assert_eq!(DomainId::parse("swe"), Some(DomainId::General));
        assert_eq!(DomainId::parse("  Frontend  "), Some(DomainId::Frontend));
    }

    #[test]
    fn domain_unknown() {
        assert_eq!(DomainId::parse("unknown"), None);
        assert_eq!(DomainId::parse(""), None);
        let err = DomainId::from_str("nope").unwrap_err();
        assert!(err.contains("unknown domain"));
        assert!(err.contains("frontend"));
    }

    #[test]
    fn theme_mode_parse_and_display() {
        assert_eq!(ThemeMode::from_str("system").unwrap(), ThemeMode::System);
        assert_eq!(ThemeMode::from_str("LIGHT").unwrap(), ThemeMode::Light);
        assert_eq!(ThemeMode::from_str("dark").unwrap(), ThemeMode::Dark);
        assert!(ThemeMode::from_str("auto").is_err());
        assert_eq!(ThemeMode::System.as_str(), "system");
        assert_eq!(ThemeMode::Light.as_str(), "light");
        assert_eq!(ThemeMode::Dark.as_str(), "dark");
    }

    #[test]
    fn portfolio_serde_roundtrip() {
        let cfg = PortfolioConfig {
            meta: PortfolioMeta {
                title: "T".into(),
                description: "D".into(),
                site_url: Some("https://x.test".into()),
            },
            domain: DomainId::Frontend,
            person: PersonInfo {
                name: "Ada".into(),
                title: "Engineer".into(),
                bio: "Bio".into(),
                location: Some("Remote".into()),
                avatar: None,
                resume_url: None,
            },
            social: SocialLinks {
                github: Some("https://github.com/ada".into()),
                email: Some("a@x.test".into()),
                ..Default::default()
            },
            greeting: Greeting {
                headline: "Hi".into(),
                subheadline: "Sub".into(),
                cta_label: Some("Go".into()),
                cta_href: Some("#projects".into()),
            },
            skills: SkillsSection {
                title: "Skills".into(),
                subtitle: None,
                groups: vec![SkillGroup {
                    name: "Lang".into(),
                    items: vec![SkillItem {
                        name: "Rust".into(),
                        level: Some(90),
                    }],
                }],
            },
            experience: vec![],
            education: vec![],
            projects: vec![ProjectItem {
                title: "P".into(),
                description: "Desc".into(),
                tags: vec!["t".into()],
                image: None,
                demo_url: None,
                repo_url: None,
                links: vec![],
                featured: true,
            }],
            achievements: vec![],
            blog: BlogConfig {
                enabled: false,
                title: None,
                subtitle: None,
                url: None,
                posts: vec![],
            },
            contact: ContactInfo {
                title: "Contact".into(),
                subtitle: None,
                email: Some("a@x.test".into()),
                booking_url: None,
            },
            theme: ThemeConfig {
                primary: "#ec4899".into(),
                mode: ThemeMode::Dark,
            },
            sections: SectionsConfig {
                skills: true,
                experience: true,
                projects: true,
                education: true,
                achievements: false,
                blog: false,
                contact: true,
            },
        };
        let json = cfg.to_pretty_json().unwrap();
        assert!(json.ends_with('\n'));
        assert!(json.contains("\"domain\": \"frontend\""));
        assert!(json.contains("\"mode\": \"dark\""));
        // Optional empty fields omitted
        assert!(!json.contains("\"avatar\""));
        let back: PortfolioConfig = serde_json::from_str(json.trim()).unwrap();
        assert_eq!(back.domain, DomainId::Frontend);
        assert_eq!(back.person.name, "Ada");
        assert_eq!(back.theme.mode, ThemeMode::Dark);
        assert!(back.projects[0].featured);
    }

    #[test]
    fn social_extra_serializes() {
        let mut extra = std::collections::BTreeMap::new();
        extra.insert("mastodon".into(), "https://mas.to/@x".into());
        let social = SocialLinks {
            extra: Some(extra),
            ..Default::default()
        };
        let v = serde_json::to_value(&social).unwrap();
        assert_eq!(v["extra"]["mastodon"], "https://mas.to/@x");
    }
}
