import { cn } from "@/lib/utils";

export function SectionHeading({
  id,
  title,
  subtitle,
  className,
}: {
  id?: string;
  title: string;
  subtitle?: string;
  className?: string;
}) {
  return (
    <div className={cn("mb-10 max-w-2xl", className)}>
      <h2
        id={id}
        className="text-2xl font-semibold tracking-tight sm:text-3xl"
      >
        {title}
      </h2>
      {subtitle ? (
        <p className="mt-2 text-muted leading-relaxed">{subtitle}</p>
      ) : null}
    </div>
  );
}
