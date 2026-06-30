"use client";

import { FormEvent, useEffect, useState } from "react";
import Link from "next/link";
import { useParams, useRouter } from "next/navigation";
import { apiFetch, apiUrl, type PortfolioRow } from "@/lib/api";

export default function AccountPortfoliosPage() {
  const { accountId } = useParams<{ accountId: string }>();
  const router = useRouter();
  const [portfolios, setPortfolios] = useState<PortfolioRow[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);
  const [file, setFile] = useState<File | null>(null);

  async function load() {
    try {
      const list = await apiFetch<PortfolioRow[]>(
        `/accounts/${accountId}/portfolios`,
      );
      setPortfolios(list);
    } catch {
      router.replace("/login");
    }
  }

  useEffect(() => {
    void load();
  }, [accountId]);

  async function onUpload(e: FormEvent) {
    e.preventDefault();
    if (!file) return;
    setUploading(true);
    setError(null);
    try {
      const body = new FormData();
      body.append("file", file);
      const res = await fetch(apiUrl(`/accounts/${accountId}/imports`), {
        method: "POST",
        credentials: "include",
        body,
      });
      if (!res.ok) {
        const j = (await res.json().catch(() => ({}))) as { error?: string };
        throw new Error(j.error || res.statusText);
      }
      setFile(null);
      await load();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Upload failed");
    } finally {
      setUploading(false);
    }
  }

  return (
    <main className="mx-auto max-w-2xl px-4 py-12">
      <Link href="/dashboard" className="text-sm underline">
        ← Accounts
      </Link>
      <h1 className="mt-4 text-2xl font-semibold">Portfolios</h1>

      <form onSubmit={onUpload} className="mt-6 flex flex-wrap items-center gap-3">
        <input
          type="file"
          accept=".csv,text/csv"
          onChange={(e) => setFile(e.target.files?.[0] ?? null)}
        />
        <button
          type="submit"
          disabled={!file || uploading}
          className="rounded bg-zinc-900 px-4 py-2 text-white disabled:opacity-50 dark:bg-zinc-100 dark:text-zinc-900"
        >
          {uploading ? "Importing…" : "Upload CSV"}
        </button>
      </form>
      {error && <p className="mt-2 text-sm text-red-600">{error}</p>}

      <ul className="mt-8 space-y-2">
        {portfolios.map((p) => (
          <li key={p.id}>
            <Link
              href={`/dashboard/${accountId}/p/${p.slug}`}
              className="block rounded border border-zinc-200 px-4 py-3 hover:bg-zinc-50 dark:border-zinc-800 dark:hover:bg-zinc-900"
            >
              <span className="font-medium">{p.person_name}</span>
              <span className="ml-2 text-sm text-zinc-500">
                {p.domain} · {p.slug}
              </span>
            </Link>
          </li>
        ))}
        {portfolios.length === 0 && (
          <li className="text-sm text-zinc-500">
            No portfolios yet — upload a CSV dataset.
          </li>
        )}
      </ul>
    </main>
  );
}
