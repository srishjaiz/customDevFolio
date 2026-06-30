//! Integration tests against real Postgres.
//!
//! Requires `DATABASE_URL` (e.g. from docker-compose):
//!   export DATABASE_URL=postgres://customfolio:customfolio@localhost:5432/customfolio
//!   cargo test -p customfolio-server --test repo_integration
//!
//! When `DATABASE_URL` is unset, tests return early so `cargo test --workspace` works offline.

use chrono::{Duration, Utc};
use customfolio_server::{
    connect, migrate, AccountRepo, ImportJobRepo, ImportJobStatus, NewAccount, NewImportJob,
    NewPortfolio, NewSession, NewUser, PortfolioRepo, SessionRepo, UserRepo,
};
use serde_json::json;
use uuid::Uuid;

fn database_url() -> Option<String> {
    std::env::var("DATABASE_URL").ok().filter(|s| !s.is_empty())
}

async fn setup_pool() -> Option<sqlx::PgPool> {
    let url = database_url()?;
    let pool = connect(&url).await.expect("connect to postgres");
    migrate(&pool).await.expect("run migrations");
    Some(pool)
}

#[tokio::test]
async fn portfolio_insert_list_get_by_account_and_slug() {
    let Some(pool) = setup_pool().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let users = UserRepo::new(&pool);
    let accounts = AccountRepo::new(&pool);
    let portfolios = PortfolioRepo::new(&pool);

    let suffix = Uuid::new_v4().simple().to_string();
    let user = users
        .create(&NewUser {
            email: format!("user-{suffix}@example.com"),
            password_hash: Some("unused-phase1".into()),
            display_name: Some("Test User".into()),
        })
        .await
        .expect("create user");

    let account = accounts
        .create(&NewAccount {
            owner_user_id: user.id,
            name: "Cohort Test".into(),
            slug: format!("cohort-{suffix}"),
            description: Some("Phase 1 integration".into()),
            default_theme: None,
        })
        .await
        .expect("create account");

    let config = json!({
        "meta": { "title": "Ada — Frontend", "description": "Sample" },
        "domain": "frontend",
        "person": { "name": "Ada Lovelace", "title": "Frontend Engineer", "bio": "Bio" },
        "social": {},
        "greeting": { "headline": "Hi", "subheadline": "There" },
        "skills": { "title": "Skills", "groups": [] },
        "experience": [],
        "education": [],
        "projects": [],
        "achievements": [],
        "blog": { "enabled": false, "posts": [] },
        "contact": { "title": "Contact" },
        "theme": { "primary": "#ec4899", "mode": "system" },
        "sections": {
            "skills": true,
            "experience": true,
            "projects": true,
            "education": true,
            "achievements": true,
            "blog": false,
            "contact": true
        }
    });

    let created = portfolios
        .create(&NewPortfolio {
            account_id: account.id,
            slug: "ada-lovelace".into(),
            domain: "frontend".into(),
            person_name: "Ada Lovelace".into(),
            config: config.clone(),
        })
        .await
        .expect("create portfolio");

    assert_eq!(created.slug, "ada-lovelace");
    assert_eq!(created.domain, "frontend");
    assert_eq!(created.person_name, "Ada Lovelace");
    assert_eq!(created.config["person"]["name"], "Ada Lovelace");

    let listed = portfolios
        .list_by_account(account.id)
        .await
        .expect("list portfolios");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, created.id);

    let got = portfolios
        .get_by_account_and_slug(account.id, "ada-lovelace")
        .await
        .expect("get by account and slug");
    assert_eq!(got.id, created.id);
    assert_eq!(got.config, config);

    let mut updated_config = config.clone();
    updated_config["person"]["title"] = json!("Staff Frontend Engineer");
    let upserted = portfolios
        .upsert(&NewPortfolio {
            account_id: account.id,
            slug: "ada-lovelace".into(),
            domain: "frontend".into(),
            person_name: "Ada Lovelace".into(),
            config: updated_config.clone(),
        })
        .await
        .expect("upsert");
    assert_eq!(upserted.id, created.id);
    assert_eq!(
        upserted.config["person"]["title"],
        "Staff Frontend Engineer"
    );

    let listed_again = portfolios.list_by_account(account.id).await.unwrap();
    assert_eq!(listed_again.len(), 1);
}

#[tokio::test]
async fn user_account_job_session_roundtrip() {
    let Some(pool) = setup_pool().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let users = UserRepo::new(&pool);
    let accounts = AccountRepo::new(&pool);
    let jobs = ImportJobRepo::new(&pool);
    let sessions = SessionRepo::new(&pool);

    let suffix = Uuid::new_v4().simple().to_string();
    let user = users
        .create(&NewUser {
            email: format!("ops-{suffix}@example.com"),
            password_hash: None,
            display_name: None,
        })
        .await
        .unwrap();

    let by_email = users.get_by_email(&user.email).await.unwrap();
    assert_eq!(by_email.id, user.id);

    let account = accounts
        .create(&NewAccount {
            owner_user_id: user.id,
            name: "Ops".into(),
            slug: format!("ops-{suffix}"),
            description: None,
            default_theme: Some(json!({"primary": "#0d9488", "mode": "dark"})),
        })
        .await
        .unwrap();

    let owned = accounts.list_by_owner(user.id).await.unwrap();
    assert!(owned.iter().any(|a| a.id == account.id));

    let job = jobs
        .create(&NewImportJob {
            account_id: account.id,
            user_id: user.id,
            source_filename: Some("people.csv".into()),
            csv_path: Some(format!("data/imports/{suffix}/source.csv")),
        })
        .await
        .unwrap();
    assert_eq!(job.status, "pending");

    let converting = jobs
        .update_status(job.id, ImportJobStatus::Converting, None)
        .await
        .unwrap();
    assert_eq!(converting.status, "converting");
    assert!(converting.started_at.is_some());

    let counted = jobs
        .update_paths_and_counts(
            job.id,
            Some(&format!("data/imports/{suffix}/data.ndjson")),
            None,
            10,
            9,
            1,
        )
        .await
        .unwrap();
    assert_eq!(counted.total_rows, 10);
    assert_eq!(counted.succeeded_rows, 9);
    assert_eq!(counted.failed_rows, 1);

    let done = jobs
        .update_status(job.id, ImportJobStatus::Completed, None)
        .await
        .unwrap();
    assert_eq!(done.status, "completed");
    assert!(done.finished_at.is_some());

    let token = format!("hash-{suffix}");
    let session = sessions
        .create(&NewSession {
            user_id: user.id,
            token_hash: token.clone(),
            expires_at: Utc::now() + Duration::hours(24),
        })
        .await
        .unwrap();

    let loaded = sessions
        .get_valid_by_token_hash(&token, Utc::now())
        .await
        .unwrap();
    assert_eq!(loaded.id, session.id);

    sessions.delete_by_id(session.id).await.unwrap();
    assert!(sessions
        .get_valid_by_token_hash(&token, Utc::now())
        .await
        .is_err());
}
