use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{RepoError, RepoResult};
use crate::models::{NewUser, User};

pub struct UserRepo<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new: &NewUser) -> RepoResult<User> {
        let email = new.email.trim();
        if email.is_empty() {
            return Err(RepoError::Invalid("email must not be empty".into()));
        }

        let row = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, password_hash, display_name)
            VALUES ($1, $2, $3)
            RETURNING id, email, password_hash, display_name, created_at, updated_at
            "#,
        )
        .bind(email)
        .bind(&new.password_hash)
        .bind(&new.display_name)
        .fetch_one(self.pool)
        .await
        .map_err(|e| {
            if RepoError::is_unique_violation(&e) {
                RepoError::Conflict("email already registered".into())
            } else {
                RepoError::Sqlx(e)
            }
        })?;

        Ok(row)
    }

    pub async fn get_by_id(&self, id: Uuid) -> RepoResult<User> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, display_name, created_at, updated_at
            FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?
        .ok_or(RepoError::NotFound)
    }

    pub async fn get_by_email(&self, email: &str) -> RepoResult<User> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, display_name, created_at, updated_at
            FROM users WHERE lower(email) = lower($1)
            "#,
        )
        .bind(email.trim())
        .fetch_optional(self.pool)
        .await?
        .ok_or(RepoError::NotFound)
    }
}
