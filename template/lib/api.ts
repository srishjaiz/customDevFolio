/** Browser client for customfolio-api (Phase 6). Free fetch only. */

const API_BASE =
  process.env.NEXT_PUBLIC_API_URL?.replace(/\/$/, "") || "http://localhost:8080";

export function apiUrl(path: string): string {
  return `${API_BASE}${path.startsWith("/") ? path : `/${path}`}`;
}

export async function apiFetch<T>(
  path: string,
  init: RequestInit = {},
): Promise<T> {
  const res = await fetch(apiUrl(path), {
    ...init,
    credentials: "include",
    headers: {
      ...(init.body && !(init.body instanceof FormData)
        ? { "Content-Type": "application/json" }
        : {}),
      ...init.headers,
    },
  });
  if (!res.ok) {
    let message = res.statusText;
    try {
      const j = (await res.json()) as { error?: string };
      if (j.error) message = j.error;
    } catch {
      /* ignore */
    }
    throw new Error(message);
  }
  if (res.status === 204) return undefined as T;
  return res.json() as Promise<T>;
}

export type UserView = {
  id: string;
  email: string;
  display_name?: string | null;
};

export type Account = {
  id: string;
  name: string;
  slug: string;
  description?: string | null;
};

export type PortfolioRow = {
  id: string;
  account_id: string;
  slug: string;
  domain: string;
  person_name: string;
  config: Record<string, unknown>;
};
