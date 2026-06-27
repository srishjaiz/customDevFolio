//! Domain profiles — mirrors `template/lib/domains.ts` for CLI defaults.

use crate::config::{
    AchievementItem, BlogConfig, BlogPost, ContactInfo, DomainId, EducationItem, ExperienceItem,
    Greeting, PersonInfo, PortfolioConfig, PortfolioMeta, ProjectItem, SectionsConfig, SkillGroup,
    SkillItem, SkillsSection, SocialLinks, ThemeConfig, ThemeMode,
};

pub struct DomainProfile {
    pub id: DomainId,
    pub label: &'static str,
    pub description: &'static str,
    pub default_sections: SectionsConfig,
    pub skill_group_hints: &'static [&'static str],
    pub default_title: &'static str,
    pub default_cta_label: &'static str,
    pub default_primary: &'static str,
}

const ALL_ON: SectionsConfig = SectionsConfig {
    skills: true,
    experience: true,
    projects: true,
    education: true,
    achievements: true,
    blog: true,
    contact: true,
};

pub fn profile(id: DomainId) -> DomainProfile {
    match id {
        DomainId::Frontend => DomainProfile {
            id,
            label: "Frontend Engineer",
            description: "UI systems, accessibility, design systems, and client performance.",
            default_sections: ALL_ON,
            skill_group_hints: &[
                "Languages",
                "Frameworks",
                "Styling",
                "Tooling",
                "Testing",
                "Design",
            ],
            default_title: "Frontend Engineer",
            default_cta_label: "View UI work",
            default_primary: "#ec4899",
        },
        DomainId::Backend => DomainProfile {
            id,
            label: "Backend Engineer",
            description: "APIs, data stores, distributed systems, and service reliability.",
            default_sections: SectionsConfig {
                blog: false,
                achievements: false,
                ..ALL_ON
            },
            skill_group_hints: &[
                "Languages",
                "Frameworks",
                "Databases",
                "Messaging",
                "Cloud",
                "Observability",
            ],
            default_title: "Backend Engineer",
            default_cta_label: "View systems",
            default_primary: "#3b82f6",
        },
        DomainId::Fullstack => DomainProfile {
            id,
            label: "Full-Stack Engineer",
            description: "End-to-end product delivery across UI and services.",
            default_sections: ALL_ON,
            skill_group_hints: &["Frontend", "Backend", "Databases", "DevOps", "Testing"],
            default_title: "Full-Stack Engineer",
            default_cta_label: "View projects",
            default_primary: "#6366f1",
        },
        DomainId::Ml => DomainProfile {
            id,
            label: "ML / AI Engineer",
            description: "Models, training pipelines, evaluation, and applied ML products.",
            default_sections: ALL_ON,
            skill_group_hints: &[
                "Languages",
                "ML frameworks",
                "Data",
                "MLOps",
                "Cloud / GPUs",
                "Research",
            ],
            default_title: "Machine Learning Engineer",
            default_cta_label: "View models & projects",
            default_primary: "#0d9488",
        },
        DomainId::Mobile => DomainProfile {
            id,
            label: "Mobile Engineer",
            description: "Native and cross-platform apps, stores, and mobile UX.",
            default_sections: SectionsConfig {
                blog: false,
                achievements: false,
                ..ALL_ON
            },
            skill_group_hints: &[
                "Platforms",
                "Languages",
                "Frameworks",
                "UI / UX",
                "Tooling",
                "Release",
            ],
            default_title: "Mobile Engineer",
            default_cta_label: "View apps",
            default_primary: "#8b5cf6",
        },
        DomainId::Devops => DomainProfile {
            id,
            label: "DevOps / SRE",
            description: "Platforms, CI/CD, infrastructure as code, and reliability.",
            default_sections: ALL_ON,
            skill_group_hints: &[
                "Cloud",
                "IaC",
                "CI/CD",
                "Containers",
                "Observability",
                "Scripting",
            ],
            default_title: "DevOps / SRE Engineer",
            default_cta_label: "View platform work",
            default_primary: "#f59e0b",
        },
        DomainId::Data => DomainProfile {
            id,
            label: "Data Engineer",
            description: "Pipelines, warehouses, orchestration, and analytics foundations.",
            default_sections: SectionsConfig {
                blog: false,
                ..ALL_ON
            },
            skill_group_hints: &[
                "Languages",
                "Warehouses",
                "Orchestration",
                "Streaming",
                "Cloud",
                "Visualization",
            ],
            default_title: "Data Engineer",
            default_cta_label: "View pipelines",
            default_primary: "#06b6d4",
        },
        DomainId::Security => DomainProfile {
            id,
            label: "Security Engineer",
            description: "AppSec, cloud security, detection, and defensive tooling.",
            default_sections: ALL_ON,
            skill_group_hints: &[
                "Languages",
                "AppSec",
                "Cloud security",
                "Detection",
                "Tooling",
                "Compliance",
            ],
            default_title: "Security Engineer",
            default_cta_label: "View security work",
            default_primary: "#ef4444",
        },
        DomainId::Game => DomainProfile {
            id,
            label: "Game Developer",
            description: "Engines, gameplay systems, graphics, and published titles.",
            default_sections: SectionsConfig {
                blog: false,
                ..ALL_ON
            },
            skill_group_hints: &[
                "Engines",
                "Languages",
                "Graphics",
                "Gameplay",
                "Tools",
                "Platforms",
            ],
            default_title: "Game Developer",
            default_cta_label: "View games",
            default_primary: "#22c55e",
        },
        DomainId::General => DomainProfile {
            id,
            label: "Software Engineer",
            description: "Balanced portfolio with all common sections enabled.",
            default_sections: ALL_ON,
            skill_group_hints: &["Languages", "Frameworks", "Tools", "Cloud", "Soft skills"],
            default_title: "Software Engineer",
            default_cta_label: "View my work",
            default_primary: "#6366f1",
        },
    }
}

pub fn list_profiles() -> Vec<DomainProfile> {
    DomainId::ALL.iter().copied().map(profile).collect()
}

/// Answers collected from flags and/or the interactive wizard.
#[derive(Debug, Clone)]
pub struct CreateAnswers {
    pub project_name: String,
    pub domain: DomainId,
    pub display_name: String,
    pub title: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
    pub resume_url: Option<String>,
    pub primary_color: Option<String>,
    pub theme_mode: ThemeMode,
    pub include_sample_content: bool,
}

impl CreateAnswers {
    /// Build a full portfolio config from answers + domain defaults.
    pub fn into_portfolio(self) -> PortfolioConfig {
        let p = profile(self.domain);
        let title = self
            .title
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| p.default_title.to_string());
        let bio = self.bio.filter(|s| !s.trim().is_empty()).unwrap_or_else(|| {
            format!(
                "I'm {}, a {} building impactful software. Edit this bio in content/portfolio.json.",
                self.display_name, title
            )
        });
        let primary = self
            .primary_color
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| p.default_primary.to_string());

        let email = self.email.filter(|s| !s.is_empty());
        let github = normalize_github(self.github);
        let linkedin = self.linkedin.filter(|s| !s.is_empty());
        let website = self.website.filter(|s| !s.is_empty());
        let resume_url = self.resume_url.filter(|s| !s.is_empty());
        let location = self.location.filter(|s| !s.is_empty());

        let meta_title = format!("{display} — {title}", display = self.display_name);
        let meta_description = format!(
            "{title} portfolio — showcase skills, experience, and projects ({domain}).",
            domain = self.domain.as_str()
        );

        let mut skills_groups: Vec<SkillGroup> = p
            .skill_group_hints
            .iter()
            .map(|name| SkillGroup {
                name: (*name).to_string(),
                items: vec![SkillItem {
                    name: "Add your skills".into(),
                    level: Some(80),
                }],
            })
            .collect();

        let (experience, education, projects, achievements, blog_posts) = if self
            .include_sample_content
        {
            sample_content(self.domain, &self.display_name, &title)
        } else {
            // Minimal placeholders so sections still render useful structure.
            skills_groups = p
                .skill_group_hints
                .iter()
                .take(2)
                .map(|name| SkillGroup {
                    name: (*name).to_string(),
                    items: vec![SkillItem {
                        name: "Your skill".into(),
                        level: Some(75),
                    }],
                })
                .collect();
            (
                vec![],
                vec![],
                vec![ProjectItem {
                    title: "Your featured project".into(),
                    description: "Replace this card in content/portfolio.json with a real project."
                        .into(),
                    tags: vec![self.domain.as_str().into()],
                    image: None,
                    demo_url: None,
                    repo_url: github.clone(),
                    links: vec![],
                    featured: true,
                }],
                vec![],
                vec![],
            )
        };

        let blog_enabled = p.default_sections.blog && !blog_posts.is_empty();

        PortfolioConfig {
            meta: PortfolioMeta {
                title: meta_title,
                description: meta_description,
                site_url: website.clone(),
            },
            domain: self.domain,
            person: PersonInfo {
                name: self.display_name.clone(),
                title: title.clone(),
                bio,
                location,
                avatar: None,
                resume_url,
            },
            social: SocialLinks {
                github: github.clone(),
                linkedin,
                twitter: None,
                email: email.clone(),
                website,
                youtube: None,
                medium: None,
                dribbble: None,
                behance: None,
                extra: None,
            },
            greeting: Greeting {
                headline: format!(
                    "Hi, I'm {} — {}.",
                    self.display_name,
                    title.to_ascii_lowercase()
                ),
                subheadline: format!(
                    "Portfolio tailored for the {} domain. Customize everything in one JSON file.",
                    p.label
                ),
                cta_label: Some(p.default_cta_label.to_string()),
                cta_href: Some("#projects".into()),
            },
            skills: SkillsSection {
                title: "Skills".into(),
                subtitle: Some("Tools and focus areas".into()),
                groups: skills_groups,
            },
            experience,
            education,
            projects,
            achievements,
            blog: BlogConfig {
                enabled: blog_enabled,
                title: Some("Writing".into()),
                subtitle: Some("Optional posts and notes".into()),
                url: None,
                posts: blog_posts,
            },
            contact: ContactInfo {
                title: "Let's work together".into(),
                subtitle: Some("Open to roles, consulting, and collaboration.".into()),
                email,
                booking_url: None,
            },
            theme: ThemeConfig {
                primary,
                mode: self.theme_mode,
            },
            sections: p.default_sections,
        }
    }
}

fn normalize_github(raw: Option<String>) -> Option<String> {
    let s = raw?.trim().to_string();
    if s.is_empty() {
        return None;
    }
    if s.starts_with("http://") || s.starts_with("https://") {
        return Some(s);
    }
    // username or github.com/user
    let user = s
        .trim_start_matches("github.com/")
        .trim_start_matches("www.github.com/");
    Some(format!("https://github.com/{user}"))
}

type SampleBundle = (
    Vec<ExperienceItem>,
    Vec<EducationItem>,
    Vec<ProjectItem>,
    Vec<AchievementItem>,
    Vec<BlogPost>,
);

fn sample_content(domain: DomainId, display_name: &str, title: &str) -> SampleBundle {
    let experience = vec![ExperienceItem {
        role: title.to_string(),
        company: "Example Company".into(),
        location: Some("Remote".into()),
        start_date: "2022".into(),
        end_date: Some("Present".into()),
        description: format!(
            "Sample role for {display_name}. Replace with real experience in portfolio.json."
        ),
        highlights: vec![
            "Shipped high-impact features end to end".into(),
            "Improved reliability and developer experience".into(),
        ],
        tech: sample_tech(domain),
        url: None,
    }];

    let education = vec![EducationItem {
        school: "Example University".into(),
        degree: "B.S.".into(),
        field: Some("Computer Science".into()),
        start_date: Some("2018".into()),
        end_date: Some("2022".into()),
        description: None,
        url: None,
    }];

    let projects = vec![
        ProjectItem {
            title: format!("{} starter project", domain.as_str()),
            description: format!(
                "Domain-flavored sample project for {}. Edit or remove in content/portfolio.json.",
                profile(domain).label
            ),
            tags: sample_tech(domain),
            image: None,
            demo_url: Some("https://example.com".into()),
            repo_url: Some("https://github.com".into()),
            links: vec![],
            featured: true,
        },
        ProjectItem {
            title: "Open source utility".into(),
            description: "A second sample project card so the grid looks populated.".into(),
            tags: vec!["Open source".into()],
            image: None,
            demo_url: None,
            repo_url: Some("https://github.com".into()),
            links: vec![],
            featured: false,
        },
    ];

    let achievements = if profile(domain).default_sections.achievements {
        vec![AchievementItem {
            title: "Sample achievement".into(),
            issuer: Some("Community".into()),
            date: Some("2024".into()),
            description: Some("Replace with talks, certs, or awards.".into()),
            url: None,
        }]
    } else {
        vec![]
    };

    let blog_posts = if profile(domain).default_sections.blog {
        vec![BlogPost {
            title: "Hello portfolio".into(),
            url: "https://example.com/blog/hello".into(),
            date: Some("2025-01".into()),
            summary: Some("Optional blog teaser — point at your real posts.".into()),
        }]
    } else {
        vec![]
    };

    (experience, education, projects, achievements, blog_posts)
}

fn sample_tech(domain: DomainId) -> Vec<String> {
    match domain {
        DomainId::Frontend => vec!["TypeScript".into(), "React".into(), "Next.js".into()],
        DomainId::Backend => vec!["Go".into(), "PostgreSQL".into(), "gRPC".into()],
        DomainId::Fullstack => vec!["TypeScript".into(), "Node.js".into(), "PostgreSQL".into()],
        DomainId::Ml => vec!["Python".into(), "PyTorch".into(), "MLOps".into()],
        DomainId::Mobile => vec!["Swift".into(), "Kotlin".into(), "React Native".into()],
        DomainId::Devops => vec!["Kubernetes".into(), "Terraform".into(), "AWS".into()],
        DomainId::Data => vec!["Spark".into(), "dbt".into(), "Airflow".into()],
        DomainId::Security => vec!["AppSec".into(), "Cloud".into(), "Detection".into()],
        DomainId::Game => vec!["Unity".into(), "C#".into(), "Godot".into()],
        DomainId::General => vec!["TypeScript".into(), "Python".into(), "Cloud".into()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_answers(domain: DomainId) -> CreateAnswers {
        CreateAnswers {
            project_name: "demo".into(),
            domain,
            display_name: "Test User".into(),
            title: None,
            bio: None,
            location: None,
            email: Some("t@example.com".into()),
            github: Some("testuser".into()),
            linkedin: None,
            website: None,
            resume_url: None,
            primary_color: None,
            theme_mode: ThemeMode::System,
            include_sample_content: true,
        }
    }

    #[test]
    fn builds_config_for_each_domain() {
        for id in DomainId::ALL {
            let cfg = base_answers(id).into_portfolio();
            assert_eq!(cfg.domain, id);
            assert!(!cfg.person.name.is_empty());
            assert_eq!(cfg.person.title, profile(id).default_title);
            assert_eq!(cfg.theme.primary, profile(id).default_primary);
            assert!(!cfg.skills.groups.is_empty());
            assert_eq!(cfg.skills.groups.len(), profile(id).skill_group_hints.len());
            assert!(!cfg.experience.is_empty());
            assert_eq!(cfg.projects.len(), 2);
            let json = cfg.to_pretty_json().unwrap();
            assert!(json.contains("\"domain\""));
            assert!(json.contains("https://github.com/testuser"));
        }
    }

    #[test]
    fn list_profiles_covers_all_domains() {
        let profiles = list_profiles();
        assert_eq!(profiles.len(), DomainId::ALL.len());
        for (p, id) in profiles.iter().zip(DomainId::ALL.iter()) {
            assert_eq!(p.id, *id);
            assert!(!p.label.is_empty());
            assert!(!p.description.is_empty());
            assert!(!p.default_title.is_empty());
            assert!(p.default_primary.starts_with('#'));
        }
    }

    #[test]
    fn domain_section_defaults() {
        assert!(!profile(DomainId::Backend).default_sections.blog);
        assert!(!profile(DomainId::Backend).default_sections.achievements);
        assert!(!profile(DomainId::Mobile).default_sections.blog);
        assert!(!profile(DomainId::Data).default_sections.blog);
        assert!(!profile(DomainId::Game).default_sections.blog);
        assert!(profile(DomainId::Frontend).default_sections.blog);
        assert!(profile(DomainId::Fullstack).default_sections.contact);
    }

    #[test]
    fn sample_content_respects_section_flags() {
        let backend = base_answers(DomainId::Backend).into_portfolio();
        assert!(backend.achievements.is_empty());
        assert!(backend.blog.posts.is_empty());
        assert!(!backend.blog.enabled);

        let frontend = base_answers(DomainId::Frontend).into_portfolio();
        assert!(!frontend.achievements.is_empty());
        assert!(!frontend.blog.posts.is_empty());
        assert!(frontend.blog.enabled);
    }

    #[test]
    fn minimal_content_path() {
        let mut a = base_answers(DomainId::Ml);
        a.include_sample_content = false;
        let cfg = a.into_portfolio();
        assert!(cfg.experience.is_empty());
        assert!(cfg.education.is_empty());
        assert!(cfg.achievements.is_empty());
        assert!(cfg.blog.posts.is_empty());
        assert!(!cfg.blog.enabled);
        assert_eq!(cfg.projects.len(), 1);
        assert!(cfg.projects[0].featured);
        assert_eq!(cfg.skills.groups.len(), 2);
        assert!(cfg.skills.groups[0].items[0].name.contains("skill"));
    }

    #[test]
    fn custom_fields_override_defaults() {
        let mut a = base_answers(DomainId::General);
        a.title = Some("Staff Engineer".into());
        a.bio = Some("Custom bio.".into());
        a.primary_color = Some("#112233".into());
        a.location = Some("Berlin".into());
        a.linkedin = Some("https://linkedin.com/in/x".into());
        a.website = Some("https://example.com".into());
        a.resume_url = Some("https://example.com/cv.pdf".into());
        a.theme_mode = ThemeMode::Dark;
        let cfg = a.into_portfolio();
        assert_eq!(cfg.person.title, "Staff Engineer");
        assert_eq!(cfg.person.bio, "Custom bio.");
        assert_eq!(cfg.theme.primary, "#112233");
        assert_eq!(cfg.theme.mode, ThemeMode::Dark);
        assert_eq!(cfg.person.location.as_deref(), Some("Berlin"));
        assert_eq!(cfg.meta.site_url.as_deref(), Some("https://example.com"));
        assert_eq!(
            cfg.person.resume_url.as_deref(),
            Some("https://example.com/cv.pdf")
        );
        assert_eq!(
            cfg.social.linkedin.as_deref(),
            Some("https://linkedin.com/in/x")
        );
        assert!(cfg.meta.title.contains("Staff Engineer"));
        assert!(cfg.greeting.headline.contains("staff engineer"));
    }

    #[test]
    fn empty_optional_fields_use_generated_bio() {
        let mut a = base_answers(DomainId::Devops);
        a.bio = Some("   ".into());
        a.title = Some("".into());
        a.email = Some("".into());
        a.github = Some("".into());
        a.primary_color = Some("".into());
        let cfg = a.into_portfolio();
        assert!(cfg.person.bio.contains("Test User"));
        assert_eq!(cfg.person.title, profile(DomainId::Devops).default_title);
        assert!(cfg.social.email.is_none());
        assert!(cfg.social.github.is_none());
        assert_eq!(cfg.theme.primary, profile(DomainId::Devops).default_primary);
    }

    #[test]
    fn normalize_github_variants() {
        assert_eq!(
            normalize_github(Some("alice".into())).as_deref(),
            Some("https://github.com/alice")
        );
        assert_eq!(
            normalize_github(Some("https://github.com/bob".into())).as_deref(),
            Some("https://github.com/bob")
        );
        assert_eq!(
            normalize_github(Some("http://github.com/carol".into())).as_deref(),
            Some("http://github.com/carol")
        );
        assert_eq!(
            normalize_github(Some("github.com/dave".into())).as_deref(),
            Some("https://github.com/dave")
        );
        assert_eq!(
            normalize_github(Some("www.github.com/eve".into())).as_deref(),
            Some("https://github.com/eve")
        );
        assert!(normalize_github(None).is_none());
        assert!(normalize_github(Some("  ".into())).is_none());
    }

    #[test]
    fn sample_tech_per_domain_non_empty() {
        for id in DomainId::ALL {
            let tech = sample_tech(id);
            assert_eq!(tech.len(), 3, "domain {id}");
        }
    }

    #[test]
    fn minimal_project_links_github_when_set() {
        let mut a = base_answers(DomainId::Security);
        a.include_sample_content = false;
        a.github = Some("secdev".into());
        let cfg = a.into_portfolio();
        assert_eq!(
            cfg.projects[0].repo_url.as_deref(),
            Some("https://github.com/secdev")
        );
    }
}
