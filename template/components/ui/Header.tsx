import { portfolio } from "@/lib/portfolio";
import { ThemeToggle } from "./ThemeToggle";
import { cn } from "@/lib/utils";

const links: { href: string; label: string; show: boolean }[] = [
  { href: "#skills", label: "Skills", show: portfolio.sections.skills },
  {
    href: "#experience",
    label: "Experience",
    show: portfolio.sections.experience,
  },
  { href: "#projects", label: "Projects", show: portfolio.sections.projects },
  {
    href: "#education",
    label: "Education",
    show: portfolio.sections.education,
  },
  {
    href: "#achievements",
    label: "Achievements",
    show: portfolio.sections.achievements,
  },
  {
    href: "#blog",
    label: "Blog",
    show: portfolio.sections.blog && portfolio.blog.enabled,
  },
  { href: "#contact", label: "Contact", show: portfolio.sections.contact },
];

export function Header() {
  const visible = links.filter((l) => l.show);

  return (
    <header className="sticky top-0 z-40 border-b border-card-border/80 bg-background/80 backdrop-blur-md">
      <div className="container-narrow flex h-14 items-center justify-between gap-4 px-4 sm:px-6 lg:px-8">
        <a
          href="#"
          className="truncate text-sm font-semibold tracking-tight"
        >
          {portfolio.person.name}
        </a>
        <nav
          className="hidden items-center gap-1 md:flex"
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
        <div className="flex items-center gap-2">
          {portfolio.person.resumeUrl ? (
            <a
              href={portfolio.person.resumeUrl}
              className="hidden rounded-xl border border-card-border px-3 py-2 text-xs font-medium sm:inline-flex hover:bg-section-alt"
              target={
                portfolio.person.resumeUrl.startsWith("http")
                  ? "_blank"
                  : undefined
              }
              rel="noopener noreferrer"
            >
              Resume
            </a>
          ) : null}
          <ThemeToggle />
        </div>
      </div>
    </header>
  );
}
