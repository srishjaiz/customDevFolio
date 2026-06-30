use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{RepoError, RepoResult};
use crate::models::{NewSession, Session};

pub struct SessionRepo<'a> {
    pool: &'a PgPool,
}

impl<'a> SessionRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new: &NewSession) -> RepoResult<Session> {
        if new.token_hash.trim().is_empty() {
            return Err(RepoError::Invalid("token_hash must not be empty".into()));
        }

        let row = sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token_hash, expires_at, created_at
            "#,
        )
        .bind(new.user_id)
        .bind(&new.token_hash)
        .bind(new.expires_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| {
            if RepoError::is_unique_violation(&e) {
                RepoError::Conflict("session token already exists".into())
            } else {
                RepoError::Sqlx(e)
            }
        })?;
        Ok(row)
    }

    pub async fn get_valid_by_token_hash(
        &self,
        token_hash: &str,
        now: DateTime<Utc>,
    ) -> RepoResult<Session> {
        sqlx::query_as::<_, Session>(
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at
            FROM sessions
            WHERE token_hash = $1 AND expires_at > $2
            "#,
        )
        .bind(token_hash)
        .bind(now)
        .fetch_optional(self.pool)
        .await?
        .ok_or(RepoError::NotFound)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        if res.rows_affected() == 0 {
            return Err(RepoError::NotFound);
        }
        Ok(())
    }
}
