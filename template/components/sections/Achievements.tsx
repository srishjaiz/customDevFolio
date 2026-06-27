import { Container } from "@/components/ui/Container";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { portfolio } from "@/lib/portfolio";

export function Achievements() {
  const items = portfolio.achievements;
  if (!items.length) return null;

  return (
    <section
      id="achievements"
      className="section-pad bg-section-alt/50"
      aria-labelledby="achievements-heading"
    >
      <Container>
        <SectionHeading
          id="achievements-heading"
          title="Achievements"
          subtitle="Talks, awards, certifications, and milestones"
        />
        <ul className="space-y-4">
          {items.map((a, i) => (
            <li key={`${a.title}-${i}`} className="card-surface p-5 sm:p-6">
              <div className="flex flex-wrap items-baseline justify-between gap-2">
                <h3 className="font-semibold">
                  {a.url ? (
                    <a
                      href={a.url}
                      className="text-primary underline-offset-4 hover:underline"
                      target="_blank"
                      rel="noopener noreferrer"
                    >
                      {a.title}
                    </a>
                  ) : (
                    a.title
                  )}
                </h3>
                <p className="text-xs text-muted">
                  {[a.issuer, a.date].filter(Boolean).join(" · ")}
                </p>
              </div>
              {a.description ? (
                <p className="mt-2 text-sm text-muted">{a.description}</p>
              ) : null}
            </li>
          ))}
        </ul>
      </Container>
    </section>
  );
}
