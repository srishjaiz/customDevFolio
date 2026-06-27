import { Container } from "@/components/ui/Container";
import { SectionHeading } from "@/components/ui/SectionHeading";
import { ButtonLink } from "@/components/ui/ButtonLink";
import { portfolio, socialEntries } from "@/lib/portfolio";

export function Contact() {
  const { contact } = portfolio;
  const email = contact.email ?? portfolio.social.email;
  const socials = socialEntries().filter((s) => s.key !== "email");

  return (
    <section
      id="contact"
      className="section-pad bg-section-alt/50"
      aria-labelledby="contact-heading"
    >
      <Container>
        <div className="card-surface p-8 sm:p-10">
          <SectionHeading
            id="contact-heading"
            title={contact.title}
            subtitle={contact.subtitle}
            className="mb-6"
          />
          <div className="flex flex-wrap gap-3">
            {email ? (
              <ButtonLink
                href={
                  email.startsWith("mailto:") ? email : `mailto:${email}`
                }
              >
                Email me
              </ButtonLink>
            ) : null}
            {contact.bookingUrl ? (
              <ButtonLink href={contact.bookingUrl} variant="secondary">
                Book a call
              </ButtonLink>
            ) : null}
          </div>
          {socials.length ? (
            <ul className="mt-6 flex flex-wrap gap-4 text-sm text-muted">
              {socials.map((s) => (
                <li key={s.key}>
                  <a
                    href={s.href}
                    className="hover:text-foreground hover:underline underline-offset-4"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    {s.label}
                  </a>
                </li>
              ))}
            </ul>
          ) : null}
        </div>
      </Container>
    </section>
  );
}
