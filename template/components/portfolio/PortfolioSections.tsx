import { Achievements } from "@/components/sections/Achievements";
import { Blog } from "@/components/sections/Blog";
import { Contact } from "@/components/sections/Contact";
import { Education } from "@/components/sections/Education";
import { Experience } from "@/components/sections/Experience";
import { Hero } from "@/components/sections/Hero";
import { Projects } from "@/components/sections/Projects";
import { Skills } from "@/components/sections/Skills";
import { isSectionEnabled } from "@/lib/portfolio";
import type { PortfolioConfig } from "@/lib/types";

/**
 * Section composition is data-driven:
 * - `sections.*` flags (often from domain presets / CLI)
 * - non-empty data gates so empty lists don't render shells
 */
export function PortfolioSections({ config }: { config: PortfolioConfig }) {
  return (
    <main>
      <Hero config={config} />
      {isSectionEnabled(config, "skills", config.skills.groups.length > 0) ? (
        <Skills config={config} />
      ) : null}
      {isSectionEnabled(config, "experience", config.experience.length > 0) ? (
        <Experience config={config} />
      ) : null}
      {isSectionEnabled(config, "projects", config.projects.length > 0) ? (
        <Projects config={config} />
      ) : null}
      {isSectionEnabled(config, "education", config.education.length > 0) ? (
        <Education config={config} />
      ) : null}
      {isSectionEnabled(
        config,
        "achievements",
        config.achievements.length > 0,
      ) ? (
        <Achievements config={config} />
      ) : null}
      {isSectionEnabled(
        config,
        "blog",
        config.blog.enabled &&
          ((config.blog.posts?.length ?? 0) > 0 || !!config.blog.url),
      ) ? (
        <Blog config={config} />
      ) : null}
      {isSectionEnabled(config, "contact", true) ? (
        <Contact config={config} />
      ) : null}
    </main>
  );
}
