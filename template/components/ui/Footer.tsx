import type { PortfolioConfig } from "@/lib/types";

export function Footer({ config }: { config: PortfolioConfig }) {
  const year = new Date().getFullYear();
  return (
    <footer className="border-t border-card-border py-10">
      <div className="container-narrow flex flex-col items-start justify-between gap-3 px-4 text-sm text-muted sm:flex-row sm:items-center sm:px-6 lg:px-8">
        <p>
          © {year} {config.person.name}. Built with{" "}
          <span className="text-foreground/80">customFolio</span> template.
        </p>
        <p className="text-xs">
          Domain preset:{" "}
          <code className="rounded bg-section-alt px-1.5 py-0.5 text-foreground/80">
            {config.domain}
          </code>
        </p>
      </div>
    </footer>
  );
}
