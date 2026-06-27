import { Hero } from "@/components/sections/Hero";
import { Skills } from "@/components/sections/Skills";
import { Experience } from "@/components/sections/Experience";
import { Projects } from "@/components/sections/Projects";
import { Education } from "@/components/sections/Education";
import { Achievements } from "@/components/sections/Achievements";
import { Blog } from "@/components/sections/Blog";
import { Contact } from "@/components/sections/Contact";
import { portfolio, isSectionEnabled } from "@/lib/portfolio";

/**
 * Section composition is data-driven:
 * - `sections.*` flags (often from domain presets / CLI)
 * - non-empty data gates so empty lists don't render shells
 */
export default function HomePage() {
  return (
    <>
      <Hero />
      {isSectionEnabled("skills", portfolio.skills.groups.length > 0) ? (
        <Skills />
      ) : null}
      {isSectionEnabled("experience", portfolio.experience.length > 0) ? (
        <Experience />
      ) : null}
      {isSectionEnabled("projects", portfolio.projects.length > 0) ? (
        <Projects />
      ) : null}
      {isSectionEnabled("education", portfolio.education.length > 0) ? (
        <Education />
      ) : null}
      {isSectionEnabled("achievements", portfolio.achievements.length > 0) ? (
        <Achievements />
      ) : null}
      {isSectionEnabled(
        "blog",
        portfolio.blog.enabled &&
          ((portfolio.blog.posts?.length ?? 0) > 0 || !!portfolio.blog.url),
      ) ? (
        <Blog />
      ) : null}
      {isSectionEnabled("contact", true) ? <Contact /> : null}
    </>
  );
}
