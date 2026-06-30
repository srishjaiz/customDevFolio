//! Postgres-backed storage and HTTP API for customFolio.
//! Free / OSS stack only — see `docs/adr/0001-free-stack.md`.

pub mod auth;
pub mod csv_convert;
pub mod db;
pub mod error;
pub mod http;
pub mod import;
pub mod models;
pub mod repos;
pub mod slug;

pub use db::{connect, migrate, PgPool};
pub use error::{RepoError, RepoResult};
pub use import::{import_ndjson_file, ImportStats};
pub use models::{
    Account, ImportJob, ImportJobStatus, NewAccount, NewImportJob, NewPortfolio, NewSession,
    NewUser, Portfolio, Session, User,
};
pub use repos::{AccountRepo, ImportJobRepo, PortfolioRepo, SessionRepo, UserRepo};
pub use slug::{is_valid_slug, normalize_slug};
