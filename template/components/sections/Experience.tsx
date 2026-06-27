import { Container } from "@/components/ui/Container";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { Badge } from "@/components/ui/Badge";
import { portfolio } from "@/lib/portfolio";
import { formatDateRange } from "@/lib/utils";

export function Experience() {
  const items = portfolio.experience;
  if (!items.length) return null;

  return (
    <section
      id="experience"
      className="section-pad"
      aria-labelledby="experience-heading"
    >
      <Container>
        <SectionHeading
          id="experience-heading"
          title="Experience"
          subtitle="Roles and impact"
        />
        <ol className="relative space-y-8 border-l border-card-border pl-6">
          {items.map((job, i) => (
            <li key={`${job.company}-${job.role}-${i}`} className="relative">
              <span
                className="absolute -left-[1.54rem] top-1.5 size-2.5 rounded-full bg-primary ring-4 ring-background"
                aria-hidden
              />
              <div className="card-surface p-5 sm:p-6">
                <div className="flex flex-wrap items-baseline justify-between gap-2">
                  <h3 className="text-lg font-semibold">
                    {job.role}
                    <span className="font-normal text-muted">
                      {" "}
                      ·{" "}
                      {job.url ? (
                        <a
                          href={job.url}
                          className="text-primary underline-offset-4 hover:underline"
                          target="_blank"
                          rel="noopener noreferrer"
                        >
                          {job.company}
                        </a>
                      ) : (
                        job.company
                      )}
                    </span>
                  </h3>
                  <p className="text-xs font-medium text-muted">
                    {formatDateRange(job.startDate, job.endDate)}
                    {job.location ? ` · ${job.location}` : ""}
                  </p>
                </div>
                <p className="mt-2 text-sm leading-relaxed text-foreground/85">
                  {job.description}
                </p>
                {job.highlights?.length ? (
                  <ul className="mt-3 list-disc space-y-1 pl-5 text-sm text-muted">
                    {job.highlights.map((h) => (
                      <li key={h}>{h}</li>
                    ))}
                  </ul>
                ) : null}
                {job.tech?.length ? (
                  <div className="mt-4 flex flex-wrap gap-2">
                    {job.tech.map((t) => (
                      <Badge key={t}>{t}</Badge>
                    ))}
                  </div>
                ) : null}
              </div>
            </li>
          ))}
        </ol>
      </Container>
    </section>
  );
}
