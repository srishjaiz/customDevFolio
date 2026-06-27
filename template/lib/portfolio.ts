import portfolioJson from "@/content/portfolio.json";
import type { PortfolioConfig, SectionsConfig } from "./types";
import { getDomainProfile } from "./domains";

/** Runtime portfolio loaded from generated/edited JSON. */
export const portfolio = portfolioJson as PortfolioConfig;

export function isSectionEnabled(
  key: keyof SectionsConfig,
  dataNonEmpty: boolean,
): boolean {
  return portfolio.sections[key] === true && dataNonEmpty;
}

export function domainLabel(): string {
  return getDomainProfile(portfolio.domain).label;
}

/** Social entries with a defined URL, for rendering icons/links. */
export function socialEntries(): { key: string; href: string; label: string }[] {
  const s = portfolio.social;
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
