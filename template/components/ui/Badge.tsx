import { cn } from "@/lib/utils";

export function Badge({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <span
      className={cn(
        "inline-flex items-center rounded-full border border-card-border bg-background px-2.5 py-0.5 text-xs font-medium text-foreground/90",
        className,
      )}
    >
      {children}
    </span>
  );
}
