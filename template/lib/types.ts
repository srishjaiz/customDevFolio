/** Supported software-engineering domains for portfolio presets and section defaults. */
export type DomainId =
  | "frontend"
  | "backend"
  | "fullstack"
  | "ml"
  | "mobile"
  | "devops"
  | "data"
  | "security"
  | "game"
  | "general";

export type ThemeMode = "system" | "light" | "dark";

export interface SectionsConfig {
  skills: boolean;
  experience: boolean;
  projects: boolean;
  education: boolean;
  achievements: boolean;
  blog: boolean;
  contact: boolean;
}

export interface SocialLinks {
  github?: string;
  linkedin?: string;
  twitter?: string;
  email?: string;
  website?: string;
  youtube?: string;
  medium?: string;
  dribbble?: string;
  behance?: string;
  /** Extra platform → URL pairs for domains that need more links. */
  extra?: Record<string, string>;
}

export interface PersonInfo {
  name: string;
  title: string;
  bio: string;
  location?: string;
  avatar?: string;
  resumeUrl?: string;
}

export interface Greeting {
  headline: string;
  subheadline: string;
  /** Primary CTA label (e.g. "View projects"). */
  ctaLabel?: string;
  /** Anchor or URL for primary CTA. */
  ctaHref?: string;
}

export interface SkillItem {
  name: string;
  /** Optional proficiency 0–100 for visual bars; omit for chips-only. */
  level?: number;
}

export interface SkillGroup {
  name: string;
  items: SkillItem[];
}

export interface SkillsSection {
  title: string;
  subtitle?: string;
  groups: SkillGroup[];
}

export interface ExperienceItem {
  role: string;
  company: string;
  location?: string;
  startDate: string;
  endDate?: string;
  description: string;
  highlights?: string[];
  tech?: string[];
  url?: string;
}

export interface EducationItem {
  school: string;
  degree: string;
  field?: string;
  startDate?: string;
  endDate?: string;
  description?: string;
  url?: string;
}

export interface ProjectItem {
  title: string;
  description: string;
  /** Short tags (languages, frameworks, platforms). */
  tags?: string[];
  image?: string;
  demoUrl?: string;
  repoUrl?: string;
  /** Domain-specific extras (app store, paper, dataset, etc.). */
  links?: { label: string; href: string }[];
  featured?: boolean;
}

export interface AchievementItem {
  title: string;
  issuer?: string;
  date?: string;
  description?: string;
  url?: string;
}

export interface BlogConfig {
  enabled: boolean;
  title?: string;
  subtitle?: string;
  /** External blog index or RSS-style list URL. */
  url?: string;
  posts?: { title: string; url: string; date?: string; summary?: string }[];
}

export interface ContactInfo {
  title: string;
  subtitle?: string;
  email?: string;
  /** Optional Calendly / booking link. */
  bookingUrl?: string;
}

export interface ThemeConfig {
  /** CSS color (hex or oklch) used as brand primary. */
  primary: string;
  mode: ThemeMode;
}

export interface PortfolioMeta {
  title: string;
  description: string;
  siteUrl?: string;
}

/**
 * Single source of truth for portfolio content.
 * The future Rust CLI will generate this file (JSON) from prompts + domain presets.
 */
export interface PortfolioConfig {
  meta: PortfolioMeta;
  domain: DomainId;
  person: PersonInfo;
  social: SocialLinks;
  greeting: Greeting;
  skills: SkillsSection;
  experience: ExperienceItem[];
  education: EducationItem[];
  projects: ProjectItem[];
  achievements: AchievementItem[];
  blog: BlogConfig;
  contact: ContactInfo;
  theme: ThemeConfig;
  sections: SectionsConfig;
}
