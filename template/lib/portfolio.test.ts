import { beforeEach, describe, expect, it, vi } from "vitest";

// portfolio.ts imports JSON at module load; mock it for unit isolation.
vi.mock("@/content/portfolio.json", () => ({
  default: {
    meta: { title: "T", description: "D" },
    domain: "fullstack",
    person: {
      name: "Test User",
      title: "Engineer",
      bio: "Bio",
    },
    social: {
      github: "https://github.com/test",
      linkedin: "https://linkedin.com/in/test",
      email: "test@example.com",
      website: "https://example.com",
      twitter: "",
      extra: { mastodon: "https://mas.to/@test" },
    },
    greeting: { headline: "Hi", subheadline: "Sub" },
    skills: { title: "Skills", groups: [] },
    experience: [{ role: "R", company: "C", startDate: "2020", description: "d" }],
    education: [],
    projects: [{ title: "P", description: "d" }],
    achievements: [],
    blog: { enabled: false, posts: [] },
    contact: { title: "Contact", email: "test@example.com" },
    theme: { primary: "#6366f1", mode: "system" },
    sections: {
      skills: true,
      experience: true,
      projects: true,
      education: true,
      achievements: false,
      blog: false,
      contact: true,
    },
  },
}));

describe("portfolio helpers", () => {
  beforeEach(() => {
    vi.resetModules();
  });

  it("domainLabel uses domain profile", async () => {
    const { domainLabel } = await import("./portfolio");
    expect(domainLabel()).toBe("Full-Stack Engineer");
  });

  it("isSectionEnabled requires flag and data", async () => {
    const { isSectionEnabled } = await import("./portfolio");
    expect(isSectionEnabled("experience", true)).toBe(true);
    expect(isSectionEnabled("experience", false)).toBe(false);
    expect(isSectionEnabled("achievements", true)).toBe(false);
    expect(isSectionEnabled("blog", false)).toBe(false);
  });

  it("socialEntries maps known links and extras", async () => {
    const { socialEntries } = await import("./portfolio");
    const entries = socialEntries();
    const keys = entries.map((e) => e.key);
    expect(keys).toContain("github");
    expect(keys).toContain("linkedin");
    expect(keys).toContain("email");
    expect(keys).toContain("website");
    expect(keys).toContain("mastodon");
    expect(keys).not.toContain("twitter");

    const email = entries.find((e) => e.key === "email");
    expect(email?.href).toBe("mailto:test@example.com");
    expect(email?.label).toBe("Email");
  });

  it("socialEntries preserves mailto emails", async () => {
    vi.doMock("@/content/portfolio.json", () => ({
      default: {
        meta: { title: "T", description: "D" },
        domain: "general",
        person: { name: "A", title: "B", bio: "C" },
        social: { email: "mailto:direct@example.com" },
        greeting: { headline: "H", subheadline: "S" },
        skills: { title: "Skills", groups: [] },
        experience: [],
        education: [],
        projects: [],
        achievements: [],
        blog: { enabled: false },
        contact: { title: "C" },
        theme: { primary: "#000", mode: "light" },
        sections: {
          skills: true,
          experience: true,
          projects: true,
          education: true,
          achievements: true,
          blog: true,
          contact: true,
        },
      },
    }));
    vi.resetModules();
    const { socialEntries } = await import("./portfolio");
    const email = socialEntries().find((e) => e.key === "email");
    expect(email?.href).toBe("mailto:direct@example.com");
  });
});
