import { PortfolioView } from "@/components/portfolio/PortfolioView";
import { portfolio } from "@/lib/portfolio";

export default function HomePage() {
  return <PortfolioView config={portfolio} />;
}
