use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{RepoError, RepoResult};
use crate::models::{NewPortfolio, Portfolio};
use crate::slug::is_valid_slug;

pub struct PortfolioRepo<'a> {
    pool: &'a PgPool,
}

impl<'a> PortfolioRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Insert a portfolio. Fails with [`RepoError::Conflict`] if `(account_id, slug)` exists.
    pub async fn create(&self, new: &NewPortfolio) -> RepoResult<Portfolio> {
        self.validate(new)?;

        let row = sqlx::query_as::<_, Portfolio>(
            r#"
            INSERT INTO portfolios (account_id, slug, domain, person_name, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, account_id, slug, domain, person_name, config, created_at, updated_at
            "#,
        )
        .bind(new.account_id)
        .bind(&new.slug)
        .bind(new.domain.trim())
        .bind(new.person_name.trim())
        .bind(&new.config)
        .fetch_one(self.pool)
        .await
        .map_err(|e| {
            if RepoError::is_unique_violation(&e) {
                RepoError::Conflict(format!(
                    "portfolio slug '{}' already exists for this account",
                    new.slug
                ))
            } else {
                RepoError::Sqlx(e)
            }
        })?;

        Ok(row)
    }

    /// Insert or update by `(account_id, slug)`. Returns the resulting row.
    pub async fn upsert(&self, new: &NewPortfolio) -> RepoResult<Portfolio> {
        self.validate(new)?;

        let row = sqlx::query_as::<_, Portfolio>(
            r#"
            INSERT INTO portfolios (account_id, slug, domain, person_name, config)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (account_id, slug) DO UPDATE SET
                domain = EXCLUDED.domain,
                person_name = EXCLUDED.person_name,
                config = EXCLUDED.config,
                updated_at = now()
            RETURNING id, account_id, slug, domain, person_name, config, created_at, updated_at
            "#,
        )
        .bind(new.account_id)
        .bind(&new.slug)
        .bind(new.domain.trim())
        .bind(new.person_name.trim())
        .bind(&new.config)
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_by_account_and_slug(
        &self,
        account_id: Uuid,
        slug: &str,
    ) -> RepoResult<Portfolio> {
        sqlx::query_as::<_, Portfolio>(
            r#"
            SELECT id, account_id, slug, domain, person_name, config, created_at, updated_at
            FROM portfolios
            WHERE account_id = $1 AND slug = $2
            "#,
        )
        .bind(account_id)
        .bind(slug)
        .fetch_optional(self.pool)
        .await?
        .ok_or(RepoError::NotFound)
    }

    pub async fn list_by_account(&self, account_id: Uuid) -> RepoResult<Vec<Portfolio>> {
        let rows = sqlx::query_as::<_, Portfolio>(
            r#"
            SELECT id, account_id, slug, domain, person_name, config, created_at, updated_at
            FROM portfolios
            WHERE account_id = $1
            ORDER BY person_name ASC, slug ASC
            "#,
        )
        .bind(account_id)
        .fetch_all(self.pool)
        .await?;
        Ok(rows)
    }

    fn validate(&self, new: &NewPortfolio) -> RepoResult<()> {
        if !is_valid_slug(&new.slug) {
            return Err(RepoError::Invalid(format!(
                "invalid portfolio slug '{}'",
                new.slug
            )));
        }
        if new.person_name.trim().is_empty() {
            return Err(RepoError::Invalid("person_name must not be empty".into()));
        }
        if new.domain.trim().is_empty() {
            return Err(RepoError::Invalid("domain must not be empty".into()));
        }
        Ok(())
    }
}
