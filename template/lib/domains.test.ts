import { describe, expect, it } from "vitest";
import {
  DOMAIN_LIST,
  DOMAIN_PROFILES,
  getDomainProfile,
  resolveSections,
} from "./domains";
import type { DomainId } from "./types";

const ALL_DOMAINS: DomainId[] = [
  "frontend",
  "backend",
  "fullstack",
  "ml",
  "mobile",
  "devops",
  "data",
  "security",
  "game",
  "general",
];

describe("DOMAIN_PROFILES", () => {
  it("defines every domain id", () => {
    for (const id of ALL_DOMAINS) {
      expect(DOMAIN_PROFILES[id]).toBeDefined();
      expect(DOMAIN_PROFILES[id].id).toBe(id);
      expect(DOMAIN_PROFILES[id].label.length).toBeGreaterThan(0);
      expect(DOMAIN_PROFILES[id].skillGroupHints.length).toBeGreaterThan(0);
      expect(DOMAIN_PROFILES[id].defaultTitle.length).toBeGreaterThan(0);
    }
  });

  it("DOMAIN_LIST matches profile values", () => {
    expect(DOMAIN_LIST).toHaveLength(ALL_DOMAINS.length);
    expect(DOMAIN_LIST.map((p) => p.id).sort()).toEqual([...ALL_DOMAINS].sort());
  });
});

describe("getDomainProfile", () => {
  it("returns profile for known domain", () => {
    expect(getDomainProfile("ml").defaultTitle).toContain("Learning");
  });

  it("falls back to general for unknown id", () => {
    // @ts-expect-error intentional invalid id for fallback path
    expect(getDomainProfile("unknown").id).toBe("general");
  });
});

describe("resolveSections", () => {
  it("returns domain defaults without overrides", () => {
    const sections = resolveSections("backend");
    expect(sections.blog).toBe(false);
    expect(sections.skills).toBe(true);
    expect(sections.contact).toBe(true);
  });

  it("merges partial overrides", () => {
    const sections = resolveSections("backend", { blog: true, achievements: false });
    expect(sections.blog).toBe(true);
    expect(sections.achievements).toBe(false);
    expect(sections.skills).toBe(true);
  });

  it("preserves frontend blog default", () => {
    expect(resolveSections("frontend").blog).toBe(true);
  });
});
