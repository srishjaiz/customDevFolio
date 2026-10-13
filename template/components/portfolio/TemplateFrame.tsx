"use client";

import { createContext, useContext, useState, type ReactNode } from "react";
import { resolveTemplateId } from "@/lib/portfolio";
import type { PortfolioConfig, TemplateId } from "@/lib/types";

type TemplateContextValue = {
  templateId: TemplateId;
  setTemplateId: (id: TemplateId) => void;
};

const TemplateContext = createContext<TemplateContextValue | null>(null);

export function TemplateFrame({
  config,
  children,
}: {
  config: PortfolioConfig;
  children: ReactNode;
}) {
  const [templateId, setTemplateId] = useState<TemplateId>(
    resolveTemplateId(config),
  );

  return (
    <TemplateContext.Provider value={{ templateId, setTemplateId }}>
      <div
        data-template={templateId}
        className={
          templateId === "terminal" ? "min-h-screen font-mono" : "min-h-screen"
        }
        style={
          templateId === "studio"
            ? ({ ["--primary" as string]: config.theme.primary })
            : undefined
        }
      >
        {children}
      </div>
    </TemplateContext.Provider>
  );
}

export function useTemplateLook(): TemplateContextValue {
  const value = useContext(TemplateContext);
  if (!value) {
    throw new Error("useTemplateLook must render inside TemplateFrame");
  }
  return value;
}
