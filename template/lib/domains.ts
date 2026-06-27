import type { DomainId, SectionsConfig } from "./types";

export interface DomainProfile {
  id: DomainId;
  label: string;
  description: string;
  /** Default section visibility for this domain. */
  defaultSections: SectionsConfig;
  /** Suggested skill group names shown as hints in the CLI later. */
  skillGroupHints: string[];
  /** Default role title placeholder. */
  defaultTitle: string;
  /** Hero CTA bias for this domain. */
  defaultCtaLabel: string;
}

const allOn: SectionsConfig = {
  skills: true,
  experience: true,
  projects: true,
  education: true,
  achievements: true,
  blog: true,
  contact: true,
};

export const DOMAIN_PROFILES: Record<DomainId, DomainProfile> = {
  frontend: {
    id: "frontend",
    label: "Frontend Engineer",
    description: "UI systems, accessibility, design systems, and client performance.",
    defaultSections: { ...allOn, blog: true },
    skillGroupHints: [
      "Languages",
      "Frameworks",
      "Styling",
      "Tooling",
      "Testing",
      "Design",
    ],
    defaultTitle: "Frontend Engineer",
    defaultCtaLabel: "View UI work",
  },
  backend: {
    id: "backend",
    label: "Backend Engineer",
    description: "APIs, data stores, distributed systems, and service reliability.",
    defaultSections: { ...allOn, blog: false },
    skillGroupHints: [
      "Languages",
      "Frameworks",
      "Databases",
      "Messaging",
      "Cloud",
      "Observability",
    ],
    defaultTitle: "Backend Engineer",
    defaultCtaLabel: "View systems",
  },
  fullstack: {
    id: "fullstack",
    label: "Full-Stack Engineer",
    description: "End-to-end product delivery across UI and services.",
    defaultSections: { ...allOn },
    skillGroupHints: [
      "Frontend",
      "Backend",
      "Databases",
      "DevOps",
      "Testing",
    ],
    defaultTitle: "Full-Stack Engineer",
    defaultCtaLabel: "View projects",
  },
  ml: {
    id: "ml",
    label: "ML / AI Engineer",
    description: "Models, training pipelines, evaluation, and applied ML products.",
    defaultSections: {
      ...allOn,
      achievements: true,
      blog: true,
    },
    skillGroupHints: [
      "Languages",
      "ML frameworks",
      "Data",
      "MLOps",
      "Cloud / GPUs",
      "Research",
    ],
    defaultTitle: "Machine Learning Engineer",
    defaultCtaLabel: "View models & projects",
  },
  mobile: {
    id: "mobile",
    label: "Mobile Engineer",
    description: "Native and cross-platform apps, stores, and mobile UX.",
    defaultSections: { ...allOn, blog: false },
    skillGroupHints: [
      "Platforms",
      "Languages",
      "Frameworks",
      "UI / UX",
      "Tooling",
      "Release",
    ],
    defaultTitle: "Mobile Engineer",
    defaultCtaLabel: "View apps",
  },
  devops: {
    id: "devops",
    label: "DevOps / SRE",
    description: "Platforms, CI/CD, infrastructure as code, and reliability.",
    defaultSections: {
      ...allOn,
      blog: true,
      achievements: true,
    },
    skillGroupHints: [
      "Cloud",
      "IaC",
      "CI/CD",
      "Containers",
      "Observability",
      "Scripting",
    ],
    defaultTitle: "DevOps / SRE Engineer",
    defaultCtaLabel: "View platform work",
  },
  data: {
    id: "data",
    label: "Data Engineer",
    description: "Pipelines, warehouses, orchestration, and analytics foundations.",
    defaultSections: { ...allOn, blog: false },
    skillGroupHints: [
      "Languages",
      "Warehouses",
      "Orchestration",
      "Streaming",
      "Cloud",
      "Visualization",
    ],
    defaultTitle: "Data Engineer",
    defaultCtaLabel: "View pipelines",
  },
  security: {
    id: "security",
    label: "Security Engineer",
    description: "AppSec, cloud security, detection, and defensive tooling.",
    defaultSections: {
      ...allOn,
      achievements: true,
      blog: true,
    },
    skillGroupHints: [
      "Languages",
      "AppSec",
      "Cloud security",
      "Detection",
      "Tooling",
      "Compliance",
    ],
    defaultTitle: "Security Engineer",
    defaultCtaLabel: "View security work",
  },
  game: {
    id: "game",
    label: "Game Developer",
    description: "Engines, gameplay systems, graphics, and published titles.",
    defaultSections: {
      ...allOn,
      education: true,
      blog: false,
    },
    skillGroupHints: [
      "Engines",
      "Languages",
      "Graphics",
      "Gameplay",
      "Tools",
      "Platforms",
    ],
    defaultTitle: "Game Developer",
    defaultCtaLabel: "View games",
  },
  general: {
    id: "general",
    label: "Software Engineer",
    description: "Balanced portfolio with all common sections enabled.",
    defaultSections: { ...allOn },
    skillGroupHints: ["Languages", "Frameworks", "Tools", "Cloud", "Soft skills"],
    defaultTitle: "Software Engineer",
    defaultCtaLabel: "View my work",
  },
};

export const DOMAIN_LIST = Object.values(DOMAIN_PROFILES);

export function getDomainProfile(id: DomainId): DomainProfile {
  return DOMAIN_PROFILES[id] ?? DOMAIN_PROFILES.general;
}

/** Merge explicit section overrides onto domain defaults. */
export function resolveSections(
  domain: DomainId,
  overrides?: Partial<SectionsConfig>,
): SectionsConfig {
  const base = { ...getDomainProfile(domain).defaultSections };
  if (!overrides) return base;
  return { ...base, ...overrides };
}
