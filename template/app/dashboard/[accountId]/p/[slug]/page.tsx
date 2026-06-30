"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { useParams, useRouter } from "next/navigation";
import { apiFetch, type PortfolioRow } from "@/lib/api";

export default function PortfolioDetailPage() {
  const { accountId, slug } = useParams<{ accountId: string; slug: string }>();
  const router = useRouter();
  const [row, setRow] = useState<PortfolioRow | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const p = await apiFetch<PortfolioRow>(
          `/accounts/${accountId}/portfolios/${slug}`,
        );
        setRow(p);
      } catch {
        router.replace("/login");
      }
    })();
  }, [accountId, slug, router]);

  if (!row) {
    return (
      <main className="mx-auto max-w-2xl px-4 py-16 text-sm text-zinc-500">
        Loading…
      </main>
    );
  }

  const cfg = row.config as {
    person?: { name?: string; title?: string; bio?: string };
    greeting?: { headline?: string; subheadline?: string };
    theme?: { primary?: string };
  };

  return (
    <main
      className="mx-auto max-w-2xl px-4 py-12"
      style={
        cfg.theme?.primary
          ? ({ ["--primary" as string]: cfg.theme.primary } as React.CSSProperties)
          : undefined
      }
    >
      <Link href={`/dashboard/${accountId}`} className="text-sm underline">
        ← Portfolios
      </Link>
      <h1 className="mt-4 text-3xl font-semibold tracking-tight">
        {cfg.person?.name ?? row.person_name}
      </h1>
      <p className="mt-1 text-lg text-zinc-600 dark:text-zinc-400">
        {cfg.person?.title}
      </p>
      <p className="mt-4 text-sm">{cfg.greeting?.headline}</p>
      <p className="mt-2 text-sm text-zinc-600 dark:text-zinc-400">
        {cfg.person?.bio}
      </p>
      <p className="mt-6 text-xs text-zinc-500">
        Domain: {row.domain} · slug: {row.slug}
      </p>
      <pre className="mt-8 overflow-auto rounded bg-zinc-100 p-4 text-xs dark:bg-zinc-900">
        {JSON.stringify(row.config, null, 2)}
      </pre>
    </main>
  );
}
