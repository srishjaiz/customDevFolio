import { afterEach, describe, expect, it, vi } from "vitest";
import {
  apiFetch,
  apiUrl,
  createAccount,
  createPortfolio,
  listAccounts,
  listPortfolios,
  login,
  logout,
  me,
  signup,
} from "./api";

describe("apiUrl", () => {
  it("joins base and path", () => {
    expect(apiUrl("/health")).toMatch(/\/health$/);
    expect(apiUrl("health")).toMatch(/\/health$/);
  });
});

describe("apiFetch", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    vi.restoreAllMocks();
  });

  it("returns JSON on ok response", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        ok: true,
        status: 200,
        json: async () => ({ id: "1", email: "a@b.c" }),
      }),
    );
    const user = await me();
    expect(user.email).toBe("a@b.c");
    expect(fetch).toHaveBeenCalledWith(
      expect.stringMatching(/\/auth\/me$/),
      expect.objectContaining({ credentials: "include" }),
    );
  });

  it("throws Error with API error message", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        ok: false,
        status: 401,
        statusText: "Unauthorized",
        json: async () => ({ error: "not authenticated" }),
      }),
    );
    await expect(me()).rejects.toThrow("not authenticated");
  });

  it("handles 204 with no body", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        ok: true,
        status: 204,
        json: async () => {
          throw new Error("no body");
        },
      }),
    );
    await expect(apiFetch("/noop")).resolves.toBeUndefined();
  });

  it("signup/login/logout send expected methods", async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ id: "u1", email: "x@y.z" }),
    });
    vi.stubGlobal("fetch", fetchMock);

    await signup("x@y.z", "password12", "X");
    await login("x@y.z", "password12");
    await logout();
    await listAccounts();
    await createAccount("Team");
    await listPortfolios("acc-1");
    await createPortfolio("acc-1", {
      person_name: "Ada Lovelace",
      domain: "fullstack",
      title: "Engineer",
    });

    const methods = fetchMock.mock.calls.map((c) => (c[1] as RequestInit).method);
    expect(methods).toEqual([
      "POST",
      "POST",
      "POST",
      undefined,
      "POST",
      undefined,
      "POST",
    ]);
  });
});
