import { Container } from "@/components/ui/Container";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { Badge } from "@/components/ui/Badge";
import { portfolio } from "@/lib/portfolio";

export function Projects() {
  const items = portfolio.projects;
  if (!items.length) return null;

  const sorted = [...items].sort(
    (a, b) => Number(b.featured) - Number(a.featured),
  );

  return (
    <section
      id="projects"
      className="section-pad bg-section-alt/50"
      aria-labelledby="projects-heading"
    >
      <Container>
        <SectionHeading
          id="projects-heading"
          title="Projects"
          subtitle="Selected work across your stack"
        />
        <div className="grid gap-5 sm:grid-cols-2">
          {sorted.map((project) => (
            <article
              key={project.title}
              className="card-surface flex flex-col p-5 sm:p-6"
            >
              <div className="flex items-start justify-between gap-3">
                <h3 className="text-lg font-semibold">{project.title}</h3>
                {project.featured ? (
                  <Badge className="shrink-0 border-primary/30 text-primary">
                    Featured
                  </Badge>
                ) : null}
              </div>
              <p className="mt-2 flex-1 text-sm leading-relaxed text-muted">
                {project.description}
              </p>
              {project.tags?.length ? (
                <div className="mt-4 flex flex-wrap gap-2">
                  {project.tags.map((t) => (
                    <Badge key={t}>{t}</Badge>
                  ))}
                </div>
              ) : null}
              <div className="mt-4 flex flex-wrap gap-3 text-sm font-medium">
                {project.demoUrl ? (
                  <a
                    href={project.demoUrl}
                    className="text-primary underline-offset-4 hover:underline"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    Demo
                  </a>
                ) : null}
                {project.repoUrl ? (
                  <a
                    href={project.repoUrl}
                    className="text-primary underline-offset-4 hover:underline"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    Source
                  </a>
                ) : null}
                {project.links?.map((l) => (
                  <a
                    key={l.href + l.label}
                    href={l.href}
                    className="text-primary underline-offset-4 hover:underline"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    {l.label}
                  </a>
                ))}
              </div>
            </article>
          ))}
        </div>
      </Container>
    </section>
  );
}
