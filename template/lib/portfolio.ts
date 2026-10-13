import portfolioJson from "@/content/portfolio.json";
import type { PortfolioConfig, SectionsConfig, TemplateId } from "./types";
import { getDomainProfile } from "./domains";

/** Runtime portfolio loaded from generated/edited JSON. */
export const portfolio = portfolioJson as PortfolioConfig;

/** Studio when the document omits templateId or names anything other than terminal. */
export function resolveTemplateId(config: PortfolioConfig): TemplateId {
  return config.templateId === "terminal" ? "terminal" : "studio";
}

export function isSectionEnabled(
  config: PortfolioConfig,
  key: keyof SectionsConfig,
  dataNonEmpty: boolean,
): boolean {
  return config.sections[key] === true && dataNonEmpty;
}

export function domainLabel(config: PortfolioConfig): string {
  return getDomainProfile(config.domain).label;
}

/** Social entries with a defined URL, for rendering icons/links. */
export function socialEntries(
  config: PortfolioConfig,
): { key: string; href: string; label: string }[] {
  const s = config.social;
  const entries: { key: string; href: string; label: string }[] = [];
  const map: [keyof typeof s, string][] = [
    ["github", "GitHub"],
    ["linkedin", "LinkedIn"],
    ["twitter", "Twitter / X"],
    ["email", "Email"],
    ["website", "Website"],
    ["youtube", "YouTube"],
    ["medium", "Medium"],
    ["dribbble", "Dribbble"],
    ["behance", "Behance"],
  ];

  for (const [key, label] of map) {
    const value = s[key];
    if (typeof value === "string" && value.length > 0) {
      const href =
        key === "email" && !value.startsWith("mailto:")
          ? `mailto:${value}`
          : value;
      entries.push({ key, href, label });
    }
  }

  if (s.extra) {
    for (const [key, href] of Object.entries(s.extra)) {
      if (href) entries.push({ key, href, label: key });
    }
  }

  return entries;
}
