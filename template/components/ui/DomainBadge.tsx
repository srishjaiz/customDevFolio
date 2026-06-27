import { domainLabel } from "@/lib/portfolio";
import { portfolio } from "@/lib/portfolio";

export function DomainBadge() {
  return (
    <span
      className="inline-flex items-center gap-1.5 rounded-full border border-card-border bg-card px-3 py-1 text-xs font-medium text-muted"
      title={`Portfolio domain: ${portfolio.domain}`}
    >
      <span
        className="size-1.5 rounded-full bg-primary"
        aria-hidden
      />
      {domainLabel()}
    </span>
  );
}
