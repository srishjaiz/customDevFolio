import { portfolio } from "@/lib/portfolio";

/**
 * Inline script avoids flash of wrong theme. Mode comes from portfolio.theme.mode.
 * Users can still toggle via ThemeToggle (stores preference in localStorage).
 */
export function ThemeScript() {
  const mode = portfolio.theme.mode;
  const primary = portfolio.theme.primary;

  const code = `
(function(){
  try {
    var stored = localStorage.getItem('cf-theme');
    var mode = ${JSON.stringify(mode)};
    var preferDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    var dark = stored === 'dark' || (!stored && (mode === 'dark' || (mode === 'system' && preferDark)));
    if (stored === 'light') dark = false;
    if (dark) document.documentElement.classList.add('dark');
    else document.documentElement.classList.remove('dark');
    document.documentElement.style.setProperty('--primary', ${JSON.stringify(primary)});
  } catch (e) {}
})();`;

  return (
    <script
      dangerouslySetInnerHTML={{ __html: code }}
      suppressHydrationWarning
    />
  );
}
