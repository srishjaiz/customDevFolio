//! Postgres-backed storage for customFolio accounts and portfolios.
//!
//! Free / OSS stack only — see `docs/adr/0001-free-stack.md`.
//! Phase 1: schema migrations + repositories (no HTTP yet).

pub mod db;
pub mod error;
pub mod models;
pub mod repos;
pub mod slug;

pub use db::{connect, migrate, PgPool};
pub use error::{RepoError, RepoResult};
pub use models::{
    Account, ImportJob, ImportJobStatus, NewAccount, NewImportJob, NewPortfolio, NewSession,
    NewUser, Portfolio, Session, User,
};
pub use repos::{AccountRepo, ImportJobRepo, PortfolioRepo, SessionRepo, UserRepo};
pub use slug::{is_valid_slug, normalize_slug};
