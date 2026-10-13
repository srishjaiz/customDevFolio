/** Browser client for customfolio-api. Free fetch only; credentials for session cookie. */

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

export type CreatePortfolioInput = {
  person_name: string;
  slug?: string;
  domain?: string;
  title?: string;
  bio?: string;
  config?: Record<string, unknown>;
};

export async function signup(
  email: string,
  password: string,
  display_name?: string,
): Promise<UserView> {
  return apiFetch<UserView>("/auth/signup", {
    method: "POST",
    body: JSON.stringify({ email, password, display_name }),
  });
}

export async function login(email: string, password: string): Promise<UserView> {
  return apiFetch<UserView>("/auth/login", {
    method: "POST",
    body: JSON.stringify({ email, password }),
  });
}

export async function logout(): Promise<void> {
  await apiFetch("/auth/logout", { method: "POST" });
}

export async function me(): Promise<UserView> {
  return apiFetch<UserView>("/auth/me");
}

export async function listAccounts(): Promise<Account[]> {
  return apiFetch<Account[]>("/accounts");
}

export async function createAccount(name: string): Promise<Account> {
  return apiFetch<Account>("/accounts", {
    method: "POST",
    body: JSON.stringify({ name }),
  });
}

export async function listPortfolios(accountId: string): Promise<PortfolioRow[]> {
  return apiFetch<PortfolioRow[]>(`/accounts/${accountId}/portfolios`);
}

export async function createPortfolio(
  accountId: string,
  input: CreatePortfolioInput,
): Promise<PortfolioRow> {
  return apiFetch<PortfolioRow>(`/accounts/${accountId}/portfolios`, {
    method: "POST",
    body: JSON.stringify(input),
  });
}

export async function getPortfolio(
  accountId: string,
  slug: string,
): Promise<PortfolioRow> {
  return apiFetch<PortfolioRow>(`/accounts/${accountId}/portfolios/${slug}`);
}
