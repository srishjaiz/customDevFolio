/** Simple className merger without extra deps. */
export function cn(...parts: Array<string | false | null | undefined>): string {
  return parts.filter(Boolean).join(" ");
}

export function formatDateRange(start?: string, end?: string): string {
  if (!start && !end) return "";
  if (start && !end) return `${start} — Present`;
  if (!start && end) return end;
  return `${start} — ${end}`;
}

/** Initials fallback when no avatar is set. */
export function initials(name: string): string {
  return name
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((p) => p[0]?.toUpperCase() ?? "")
    .join("");
}
