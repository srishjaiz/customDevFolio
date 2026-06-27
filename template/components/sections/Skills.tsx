import { Container } from "@/components/ui/Container";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { Badge } from "@/components/ui/Badge";
import { portfolio } from "@/lib/portfolio";

export function Skills() {
  const { skills } = portfolio;
  if (!skills.groups.length) return null;

  return (
    <section
      id="skills"
      className="section-pad bg-section-alt/50"
      aria-labelledby="skills-heading"
    >
      <Container>
        <SectionHeading
          id="skills-heading"
          title={skills.title}
          subtitle={skills.subtitle}
        />
        <div className="grid gap-6 sm:grid-cols-2">
          {skills.groups.map((group) => (
            <div key={group.name} className="card-surface p-5 sm:p-6">
              <h3 className="text-sm font-semibold uppercase tracking-wide text-muted">
                {group.name}
              </h3>
              <ul className="mt-4 space-y-3">
                {group.items.map((item) => (
                  <li key={item.name}>
                    <div className="flex items-center justify-between gap-3 text-sm">
                      <span className="font-medium">{item.name}</span>
                      {typeof item.level === "number" ? (
                        <span className="text-xs text-muted">{item.level}%</span>
                      ) : (
                        <Badge>{item.name}</Badge>
                      )}
                    </div>
                    {typeof item.level === "number" ? (
                      <div
                        className="mt-1.5 h-1.5 overflow-hidden rounded-full bg-section-alt"
                        role="presentation"
                      >
                        <div
                          className="h-full rounded-full bg-primary"
                          style={{
                            width: `${Math.min(100, Math.max(0, item.level))}%`,
                          }}
                        />
                      </div>
                    ) : null}
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
      </Container>
    </section>
  );
}
