"use client";

import { FormEvent, useCallback, useEffect, useState } from "react";
import Link from "next/link";
import { useParams, useRouter } from "next/navigation";
import {
  apiUrl,
  createPortfolio,
  listPortfolios,
  type PortfolioRow,
} from "@/lib/api";

const DOMAINS = [
  "general",
  "frontend",
  "backend",
  "fullstack",
  "ml",
  "mobile",
  "devops",
  "data",
  "security",
  "game",
] as const;

export default function AccountPortfoliosPage() {
  const { accountId } = useParams<{ accountId: string }>();
  const router = useRouter();
  const [portfolios, setPortfolios] = useState<PortfolioRow[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);
  const [creating, setCreating] = useState(false);
  const [file, setFile] = useState<File | null>(null);

  const [personName, setPersonName] = useState("");
  const [slug, setSlug] = useState("");
  const [domain, setDomain] = useState<string>("fullstack");
  const [title, setTitle] = useState("");
  const [bio, setBio] = useState("");

  const load = useCallback(async () => {
    try {
      const list = await listPortfolios(accountId);
      setPortfolios(list);
    } catch {
      router.replace("/login");
    }
  }, [accountId, router]);

  useEffect(() => {
    void load();
  }, [load]);

  async function onCreate(e: FormEvent) {
    e.preventDefault();
    setCreating(true);
    setError(null);
    try {
      const row = await createPortfolio(accountId, {
        person_name: personName,
        slug: slug.trim() || undefined,
        domain,
        title: title.trim() || undefined,
        bio: bio.trim() || undefined,
      });
      setPortfolios((prev) =>
        [...prev, row].sort((a, b) => a.person_name.localeCompare(b.person_name)),
      );
      setPersonName("");
      setSlug("");
      setTitle("");
      setBio("");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Create failed");
    } finally {
      setCreating(false);
    }
  }

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
      <p className="mt-1 text-sm text-zinc-600 dark:text-zinc-400">
        Create many portfolios under this account — one per person or brand —
        or bulk-import from CSV.
      </p>

      <section className="mt-8 rounded-lg border border-zinc-200 p-4 dark:border-zinc-800">
        <h2 className="text-lg font-medium">Create portfolio</h2>
        <form onSubmit={onCreate} className="mt-4 space-y-3">
          <div className="grid gap-3 sm:grid-cols-2">
            <label className="block text-sm">
              <span className="text-zinc-600 dark:text-zinc-400">Person name *</span>
              <input
                required
                className="mt-1 w-full rounded border border-zinc-300 px-3 py-2 dark:border-zinc-700 dark:bg-zinc-900"
                value={personName}
                onChange={(e) => setPersonName(e.target.value)}
                placeholder="Ada Lovelace"
              />
            </label>
            <label className="block text-sm">
              <span className="text-zinc-600 dark:text-zinc-400">Slug (optional)</span>
              <input
                className="mt-1 w-full rounded border border-zinc-300 px-3 py-2 dark:border-zinc-700 dark:bg-zinc-900"
                value={slug}
                onChange={(e) => setSlug(e.target.value)}
                placeholder="ada-lovelace"
              />
            </label>
            <label className="block text-sm">
              <span className="text-zinc-600 dark:text-zinc-400">Domain</span>
              <select
                className="mt-1 w-full rounded border border-zinc-300 px-3 py-2 dark:border-zinc-700 dark:bg-zinc-900"
                value={domain}
                onChange={(e) => setDomain(e.target.value)}
              >
                {DOMAINS.map((d) => (
                  <option key={d} value={d}>
                    {d}
                  </option>
                ))}
              </select>
            </label>
            <label className="block text-sm">
              <span className="text-zinc-600 dark:text-zinc-400">Title</span>
              <input
                className="mt-1 w-full rounded border border-zinc-300 px-3 py-2 dark:border-zinc-700 dark:bg-zinc-900"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="Full-Stack Engineer"
              />
            </label>
          </div>
          <label className="block text-sm">
            <span className="text-zinc-600 dark:text-zinc-400">Bio</span>
            <textarea
              rows={2}
              className="mt-1 w-full rounded border border-zinc-300 px-3 py-2 dark:border-zinc-700 dark:bg-zinc-900"
              value={bio}
              onChange={(e) => setBio(e.target.value)}
              placeholder="Short intro for this portfolio"
            />
          </label>
          <button
            type="submit"
            disabled={creating || !personName.trim()}
            className="rounded bg-zinc-900 px-4 py-2 text-white disabled:opacity-50 dark:bg-zinc-100 dark:text-zinc-900"
          >
            {creating ? "Creating…" : "Create portfolio"}
          </button>
        </form>
      </section>

      <section className="mt-8 rounded-lg border border-zinc-200 p-4 dark:border-zinc-800">
        <h2 className="text-lg font-medium">Bulk import (CSV)</h2>
        <p className="mt-1 text-xs text-zinc-500">
          Streams CSV → NDJSON on disk → Postgres. Optional for large cohorts.
        </p>
        <form
          onSubmit={onUpload}
          className="mt-4 flex flex-wrap items-center gap-3"
        >
          <input
            type="file"
            accept=".csv,text/csv"
            onChange={(e) => setFile(e.target.files?.[0] ?? null)}
          />
          <button
            type="submit"
            disabled={!file || uploading}
            className="rounded border border-zinc-300 px-4 py-2 text-sm disabled:opacity-50 dark:border-zinc-700"
          >
            {uploading ? "Importing…" : "Upload CSV"}
          </button>
        </form>
      </section>

      {error && <p className="mt-4 text-sm text-red-600">{error}</p>}

      <section className="mt-10">
        <h2 className="text-lg font-medium">
          All portfolios ({portfolios.length})
        </h2>
        <ul className="mt-4 space-y-2">
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
              No portfolios yet — create one above or upload a CSV.
            </li>
          )}
        </ul>
      </section>
    </main>
  );
}
