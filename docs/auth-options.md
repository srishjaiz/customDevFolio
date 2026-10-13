# Auth options for multi-portfolio customFolio

**Chosen for this app:** email + password with **httpOnly session cookies only**
(no JWT in localStorage, no OAuth required). Passwords are argon2id; sessions
live in Postgres (`users` / `sessions`); the browser sends
`customfolio_session` with `credentials: "include"`.

Below are optional future paths if you later want to extend that stack.

## What you have today (default — use this)

| Piece | Implementation |
|-------|----------------|
| Identity | Email + password (min 8 chars) |
| Password storage | argon2id via free Rust crate |
| Session | Random token, hashed in DB, `customfolio_session` httpOnly cookie |
| Scope | User → owns **accounts** → each account has many **portfolios** |

Good for self-host, cohorts, agencies, and internal tools. No SaaS bill.

---

## Options to consider

### 1. Keep password + session (current) — **recommended for free stack**

- **Pros:** No vendor lock-in, works offline/self-host, simple mental model.
- **Cons:** You own reset-password, MFA, and abuse protection.
- **When:** Default for this repo and ADR free-stack policy.

### 2. Magic link / passwordless email

- Send one-time login links (SMTP or free tier Resend/Mailgun).
- **Pros:** No password UX friction.
- **Cons:** Needs reliable email; slightly more moving parts.
- **Fit:** Student/cohort onboarding where passwords are painful.

### 3. OAuth / OIDC (Google, GitHub, Microsoft)

- Free provider apps; libraries: NextAuth/Auth.js (Next) or OIDC in Axum.
- **Pros:** Great for “log in with GitHub” developer portfolios.
- **Cons:** Provider setup; still need local user row mapping; not fully offline.
- **Fit:** Public product where users already have GitHub/Google.

### 4. Auth.js (NextAuth) on the Next app only

- Session in Next; call API with JWT or session token.
- **Pros:** Huge ecosystem (providers, adapters).
- **Cons:** Splits auth between Next and Rust unless you consolidate.
- **Fit:** If the UI becomes the primary product surface.

### 5. Keycloak / Authentik / Pocket ID (self-hosted IdP)

- Full SSO, OIDC, optional MFA.
- **Pros:** Free, enterprise-grade controls.
- **Cons:** Ops burden (another service).
- **Fit:** Org deploy with many internal apps.

### 6. Clerk / Auth0 / Supabase Auth (hosted)

- Fast to ship, polished UI components.
- **Pros:** MFA, social, dashboards out of the box.
- **Cons:** **Paid at scale**, data in third party — conflicts with free-stack ADR.
- **Fit:** Only if you intentionally leave the free-stack constraint.

### 7. Passkeys / WebAuthn

- Device biometrics; free libraries exist.
- **Pros:** Strong phishing resistance.
- **Cons:** Harder fallback UX; secondary factor often still needed.
- **Fit:** Add later as second factor on top of sessions.

---

## Recommendation for this Next multi-portfolio app

| Goal | Choose |
|------|--------|
| Stay free & self-host | **Current email/password + cookie sessions** |
| Better public signup | Add **GitHub OAuth** beside passwords |
| Enterprise / school SSO | Put **Keycloak/Authentik** in front; map OIDC `sub` → `users.id` |
| Fastest commercial launch | Hosted Auth (know the ADR trade-off) |

Do **not** store JWTs in `localStorage` for this app if cookies work — the API
already uses httpOnly cookies + CORS credentials for the Next origin.

Related: [multi-portfolio.md](./multi-portfolio.md) · [adr/0001-free-stack.md](./adr/0001-free-stack.md)
