"use client";

import { useTemplateLook } from "@/components/portfolio/TemplateFrame";
import type { TemplateId } from "@/lib/types";
import { cn } from "@/lib/utils";

const OPTIONS: { id: TemplateId; label: string }[] = [
  { id: "studio", label: "Studio" },
  { id: "terminal", label: "Terminal" },
];

/** Switches the shell in memory. The document underneath does not refetch. */
export function TemplateSwitcher() {
  const { templateId, setTemplateId } = useTemplateLook();

  return (
    <div
      role="group"
      aria-label="Portfolio look"
      className="flex items-center rounded-xl border border-card-border p-0.5"
    >
      {OPTIONS.map((option) => {
        const active = templateId === option.id;
        return (
          <button
            key={option.id}
            type="button"
            aria-pressed={active}
            onClick={() => setTemplateId(option.id)}
            className={cn(
              "rounded-lg px-2.5 py-1.5 text-xs font-medium transition",
              active
                ? "bg-primary text-primary-foreground"
                : "text-muted hover:text-foreground",
            )}
          >
            {option.label}
          </button>
        );
      })}
    </div>
  );
}
