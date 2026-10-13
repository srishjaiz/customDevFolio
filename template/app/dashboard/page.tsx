"use client";

import { FormEvent, useEffect, useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import {
  createAccount,
  listAccounts,
  logout as apiLogout,
  me,
  type Account,
  type UserView,
} from "@/lib/api";

export default function DashboardPage() {
  const router = useRouter();
  const [user, setUser] = useState<UserView | null>(null);
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const u = await me();
        setUser(u);
        setAccounts(await listAccounts());
      } catch {
        router.replace("/login");
      }
    })();
  }, [router]);

  async function onCreateAccount(e: FormEvent) {
    e.preventDefault();
    setError(null);
    try {
      const a = await createAccount(name);
      setAccounts((prev) => [...prev, a]);
      setName("");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed");
    }
  }

  async function onLogout() {
    await apiLogout();
    router.push("/login");
  }

  if (!user) {
    return (
      <main className="mx-auto max-w-2xl px-4 py-16 text-sm text-zinc-500">
        Loading…
      </main>
    );
  }

  return (
    <main className="mx-auto max-w-2xl px-4 py-12">
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-semibold">Multi-portfolio dashboard</h1>
          <p className="text-sm text-zinc-600 dark:text-zinc-400">
            {user.email}
            {user.display_name ? ` · ${user.display_name}` : ""}
          </p>
        </div>
        <button type="button" onClick={onLogout} className="text-sm underline">
          Log out
        </button>
      </div>

      <p className="mt-4 text-sm text-zinc-600 dark:text-zinc-400">
        An <strong>account</strong> groups many portfolios (e.g. a team, cohort,
        or agency). Create accounts here, then add as many portfolios as you
        need under each.
      </p>

      <section className="mt-10">
        <h2 className="text-lg font-medium">Your accounts</h2>
        <ul className="mt-4 space-y-2">
          {accounts.map((a) => (
            <li key={a.id}>
              <Link
                href={`/dashboard/${a.id}`}
                className="block rounded border border-zinc-200 px-4 py-3 hover:bg-zinc-50 dark:border-zinc-800 dark:hover:bg-zinc-900"
              >
                <span className="font-medium">{a.name}</span>
                <span className="ml-2 text-sm text-zinc-500">{a.slug}</span>
              </Link>
            </li>
          ))}
          {accounts.length === 0 && (
            <li className="text-sm text-zinc-500">
              No accounts yet — create one to start adding portfolios.
            </li>
          )}
        </ul>

        <form onSubmit={onCreateAccount} className="mt-6 flex gap-2">
          <input
            required
            placeholder="New account name (e.g. Cohort 2026)"
            className="flex-1 rounded border border-zinc-300 px-3 py-2 dark:border-zinc-700 dark:bg-zinc-900"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
          <button
            type="submit"
            className="rounded bg-zinc-900 px-4 py-2 text-white dark:bg-zinc-100 dark:text-zinc-900"
          >
            Create
          </button>
        </form>
        {error && <p className="mt-2 text-sm text-red-600">{error}</p>}
      </section>
    </main>
  );
}
