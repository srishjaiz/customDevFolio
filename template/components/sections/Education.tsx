import { Container } from "@/components/ui/Container";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { portfolio } from "@/lib/portfolio";
import { formatDateRange } from "@/lib/utils";

export function Education() {
  const items = portfolio.education;
  if (!items.length) return null;

  return (
    <section
      id="education"
      className="section-pad"
      aria-labelledby="education-heading"
    >
      <Container>
        <SectionHeading id="education-heading" title="Education" />
        <ul className="grid gap-4 sm:grid-cols-2">
          {items.map((ed, i) => (
            <li key={`${ed.school}-${i}`} className="card-surface p-5 sm:p-6">
              <h3 className="font-semibold">
                {ed.url ? (
                  <a
                    href={ed.url}
                    className="text-primary underline-offset-4 hover:underline"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    {ed.school}
                  </a>
                ) : (
                  ed.school
                )}
              </h3>
              <p className="mt-1 text-sm text-foreground/90">
                {ed.degree}
                {ed.field ? ` · ${ed.field}` : ""}
              </p>
              {(ed.startDate || ed.endDate) && (
                <p className="mt-1 text-xs text-muted">
                  {formatDateRange(ed.startDate, ed.endDate)}
                </p>
              )}
              {ed.description ? (
                <p className="mt-2 text-sm text-muted">{ed.description}</p>
              ) : null}
            </li>
          ))}
        </ul>
      </Container>
    </section>
  );
}
