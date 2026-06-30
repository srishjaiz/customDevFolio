use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{RepoError, RepoResult};
use crate::models::{Account, NewAccount};
use crate::slug::is_valid_slug;

pub struct AccountRepo<'a> {
    pool: &'a PgPool,
}

impl<'a> AccountRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new: &NewAccount) -> RepoResult<Account> {
        let name = new.name.trim();
        if name.is_empty() {
            return Err(RepoError::Invalid("account name must not be empty".into()));
        }
        if !is_valid_slug(&new.slug) {
            return Err(RepoError::Invalid(format!(
                "invalid account slug '{}'",
                new.slug
            )));
        }

        let theme = new
            .default_theme
            .clone()
            .unwrap_or_else(|| json!({"primary": "#6366f1", "mode": "system"}));

        let row = sqlx::query_as::<_, Account>(
            r#"
            INSERT INTO accounts (owner_user_id, name, slug, description, default_theme)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, owner_user_id, name, slug, description, default_theme,
                      created_at, updated_at
            "#,
        )
        .bind(new.owner_user_id)
        .bind(name)
        .bind(&new.slug)
        .bind(&new.description)
        .bind(theme)
        .fetch_one(self.pool)
        .await
        .map_err(|e| {
            if RepoError::is_unique_violation(&e) {
                RepoError::Conflict(format!("account slug '{}' already exists", new.slug))
            } else {
                RepoError::Sqlx(e)
            }
        })?;

        Ok(row)
    }

    pub async fn get_by_id(&self, id: Uuid) -> RepoResult<Account> {
        sqlx::query_as::<_, Account>(
            r#"
            SELECT id, owner_user_id, name, slug, description, default_theme,
                   created_at, updated_at
            FROM accounts WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?
        .ok_or(RepoError::NotFound)
    }

    pub async fn get_by_slug(&self, slug: &str) -> RepoResult<Account> {
        sqlx::query_as::<_, Account>(
            r#"
            SELECT id, owner_user_id, name, slug, description, default_theme,
                   created_at, updated_at
            FROM accounts WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(self.pool)
        .await?
        .ok_or(RepoError::NotFound)
    }

    pub async fn list_by_owner(&self, owner_user_id: Uuid) -> RepoResult<Vec<Account>> {
        let rows = sqlx::query_as::<_, Account>(
            r#"
            SELECT id, owner_user_id, name, slug, description, default_theme,
                   created_at, updated_at
            FROM accounts
            WHERE owner_user_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(owner_user_id)
        .fetch_all(self.pool)
        .await?;
        Ok(rows)
    }
}
