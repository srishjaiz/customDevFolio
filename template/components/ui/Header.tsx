import type { ReactNode } from "react";
import type { PortfolioConfig } from "@/lib/types";
import { ThemeToggle } from "./ThemeToggle";
import { cn } from "@/lib/utils";

export function Header({
  config,
  extra,
}: {
  config: PortfolioConfig;
  extra?: ReactNode;
}) {
  const links: { href: string; label: string; show: boolean }[] = [
    { href: "#skills", label: "Skills", show: config.sections.skills },
    {
      href: "#experience",
      label: "Experience",
      show: config.sections.experience,
    },
    { href: "#projects", label: "Projects", show: config.sections.projects },
    {
      href: "#education",
      label: "Education",
      show: config.sections.education,
    },
    {
      href: "#achievements",
      label: "Achievements",
      show: config.sections.achievements,
    },
    {
      href: "#blog",
      label: "Blog",
      show: config.sections.blog && config.blog.enabled,
    },
    { href: "#contact", label: "Contact", show: config.sections.contact },
  ];
  const visible = links.filter((l) => l.show);

  return (
    <header className="sticky top-0 z-40 border-b border-card-border/80 bg-background/80 backdrop-blur-md">
      <div className="container-narrow flex min-h-14 flex-wrap items-center gap-x-3 gap-y-2 px-4 py-2 sm:px-6 lg:px-8">
        <a
          href="#"
          className="shrink-0 truncate text-sm font-semibold tracking-tight"
        >
          {config.person.name}
        </a>
        <nav
          className="hidden min-w-0 flex-1 flex-wrap items-center gap-1 md:flex"
          aria-label="Primary"
        >
          {visible.map((l) => (
            <a
              key={l.href}
              href={l.href}
              className={cn(
                "rounded-lg px-2.5 py-1.5 text-xs font-medium text-muted transition hover:bg-section-alt hover:text-foreground",
              )}
            >
              {l.label}
            </a>
          ))}
        </nav>
        <div className="flex w-full flex-wrap items-center gap-2 md:ml-auto md:w-auto">
          {config.person.resumeUrl ? (
            <a
              href={config.person.resumeUrl}
              className="hidden rounded-xl border border-card-border px-3 py-2 text-xs font-medium sm:inline-flex hover:bg-section-alt"
              target={
                config.person.resumeUrl.startsWith("http")
                  ? "_blank"
                  : undefined
              }
              rel="noopener noreferrer"
            >
              Resume
            </a>
          ) : null}
          <a
            href="/dashboard"
            className="rounded-xl border border-card-border px-3 py-2 text-xs font-medium hover:bg-section-alt"
          >
            Multi-portfolio
          </a>
          {extra}
          <ThemeToggle />
        </div>
      </div>
    </header>
  );
}
