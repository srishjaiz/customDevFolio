import { DomainBadge } from "@/components/ui/DomainBadge";
import { ButtonLink } from "@/components/ui/ButtonLink";
import { Container } from "@/components/ui/Container";
import { portfolio, socialEntries } from "@/lib/portfolio";
import { initials } from "@/lib/utils";

export function Hero() {
  const { person, greeting } = portfolio;
  const socials = socialEntries();

  return (
    <section className="section-pad border-b border-card-border/60" id="top">
      <Container>
        <div className="flex flex-col gap-10 md:flex-row md:items-start md:justify-between">
          <div className="max-w-2xl space-y-6">
            <DomainBadge />
            <div className="space-y-3">
              <p className="text-sm font-medium text-primary">
                {person.title}
                {person.location ? (
                  <span className="text-muted"> · {person.location}</span>
                ) : null}
              </p>
              <h1 className="text-3xl font-semibold tracking-tight sm:text-4xl lg:text-[2.75rem] lg:leading-tight">
                {greeting.headline}
              </h1>
              <p className="text-base leading-relaxed text-muted sm:text-lg">
                {greeting.subheadline}
              </p>
              <p className="text-sm leading-relaxed text-foreground/80">
                {person.bio}
              </p>
            </div>
            <div className="flex flex-wrap gap-3">
              {greeting.ctaHref && greeting.ctaLabel ? (
                <ButtonLink href={greeting.ctaHref}>
                  {greeting.ctaLabel}
                </ButtonLink>
              ) : null}
              {person.resumeUrl ? (
                <ButtonLink href={person.resumeUrl} variant="secondary">
                  Resume
                </ButtonLink>
              ) : null}
              {portfolio.sections.contact ? (
                <ButtonLink href="#contact" variant="ghost">
                  Contact
                </ButtonLink>
              ) : null}
            </div>
            {socials.length > 0 ? (
              <ul className="flex flex-wrap gap-x-4 gap-y-2 pt-2 text-sm">
                {socials.map((s) => (
                  <li key={s.key}>
                    <a
                      href={s.href}
                      className="text-muted underline-offset-4 transition hover:text-foreground hover:underline"
                      target={
                        s.href.startsWith("http") ? "_blank" : undefined
                      }
                      rel="noopener noreferrer"
                    >
                      {s.label}
                    </a>
                  </li>
                ))}
              </ul>
            ) : null}
          </div>
          <div className="shrink-0">
            <div
              className="flex size-28 items-center justify-center rounded-3xl border border-card-border bg-card text-2xl font-semibold text-primary shadow-sm sm:size-36 sm:text-3xl"
              aria-hidden={!person.avatar}
            >
              {person.avatar ? (
                // eslint-disable-next-line @next/next/no-img-element
                <img
                  src={person.avatar}
                  alt=""
                  className="size-full rounded-3xl object-cover"
                />
              ) : (
                initials(person.name)
              )}
            </div>
          </div>
        </div>
      </Container>
    </section>
  );
}
