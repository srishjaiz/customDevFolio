import { TemplateFrame } from "@/components/portfolio/TemplateFrame";
import { TemplateSwitcher } from "@/components/portfolio/TemplateSwitcher";
import { PortfolioSections } from "@/components/portfolio/PortfolioSections";
import { Footer } from "@/components/ui/Footer";
import { Header } from "@/components/ui/Header";
import type { PortfolioConfig } from "@/lib/types";

/** Renders one portfolio document. The shell reads templateId; sections read the rest. */
export function PortfolioView({ config }: { config: PortfolioConfig }) {
  return (
    <TemplateFrame config={config}>
      <Header config={config} extra={<TemplateSwitcher />} />
      <PortfolioSections config={config} />
      <Footer config={config} />
    </TemplateFrame>
  );
}
