-- Phase 1 schema: users, accounts, portfolios, import_jobs, sessions
-- Free stack: PostgreSQL only. See docs/adr/0001-free-stack.md

CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Identity (password_hash filled in Phase 4 auth)
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email           TEXT NOT NULL,
    password_hash   TEXT,
    display_name    TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT users_email_nonempty CHECK (length(trim(email)) > 0)
);

CREATE UNIQUE INDEX users_email_lower_uidx ON users (lower(email));

-- Multi-portfolio shell owned by a user
CREATE TABLE accounts (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_user_id   UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    slug            TEXT NOT NULL,
    description     TEXT,
    default_theme   JSONB NOT NULL DEFAULT '{"primary":"#6366f1","mode":"system"}'::jsonb,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT accounts_name_nonempty CHECK (length(trim(name)) > 0),
    CONSTRAINT accounts_slug_format CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$')
);

CREATE UNIQUE INDEX accounts_slug_uidx ON accounts (slug);
CREATE INDEX accounts_owner_user_id_idx ON accounts (owner_user_id);

-- One portfolio document per (account, slug); full config in JSONB
CREATE TABLE portfolios (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id      UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    slug            TEXT NOT NULL,
    domain          TEXT NOT NULL,
    person_name     TEXT NOT NULL,
    config          JSONB NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT portfolios_slug_format CHECK (slug ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'),
    CONSTRAINT portfolios_person_name_nonempty CHECK (length(trim(person_name)) > 0),
    CONSTRAINT portfolios_domain_nonempty CHECK (length(trim(domain)) > 0)
);

CREATE UNIQUE INDEX portfolios_account_slug_uidx ON portfolios (account_id, slug);
CREATE INDEX portfolios_account_id_idx ON portfolios (account_id);
CREATE INDEX portfolios_domain_idx ON portfolios (domain);
CREATE INDEX portfolios_person_name_idx ON portfolios (person_name);

-- CSV → NDJSON → DB import tracking (Phases 2–3 / 5)
CREATE TABLE import_jobs (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id          UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    user_id             UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    status              TEXT NOT NULL DEFAULT 'pending',
    source_filename     TEXT,
    csv_path            TEXT,
    ndjson_path         TEXT,
    errors_path         TEXT,
    total_rows          BIGINT NOT NULL DEFAULT 0,
    succeeded_rows      BIGINT NOT NULL DEFAULT 0,
    failed_rows         BIGINT NOT NULL DEFAULT 0,
    error_message       TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    started_at          TIMESTAMPTZ,
    finished_at         TIMESTAMPTZ,
    CONSTRAINT import_jobs_status_check CHECK (
        status IN (
            'pending',
            'converting',
            'importing',
            'completed',
            'failed'
        )
    )
);

CREATE INDEX import_jobs_account_id_idx ON import_jobs (account_id);
CREATE INDEX import_jobs_user_id_idx ON import_jobs (user_id);
CREATE INDEX import_jobs_status_idx ON import_jobs (status);

-- Server-side sessions for cookie auth (Phase 4)
CREATE TABLE sessions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token_hash      TEXT NOT NULL,
    expires_at      TIMESTAMPTZ NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT sessions_token_hash_nonempty CHECK (length(trim(token_hash)) > 0)
);

CREATE UNIQUE INDEX sessions_token_hash_uidx ON sessions (token_hash);
CREATE INDEX sessions_user_id_idx ON sessions (user_id);
CREATE INDEX sessions_expires_at_idx ON sessions (expires_at);
