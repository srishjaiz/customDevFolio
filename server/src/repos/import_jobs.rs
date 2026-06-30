use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{RepoError, RepoResult};
use crate::models::{ImportJob, ImportJobStatus, NewImportJob};

pub struct ImportJobRepo<'a> {
    pool: &'a PgPool,
}

impl<'a> ImportJobRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new: &NewImportJob) -> RepoResult<ImportJob> {
        let row = sqlx::query_as::<_, ImportJob>(
            r#"
            INSERT INTO import_jobs (account_id, user_id, source_filename, csv_path, status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, account_id, user_id, status, source_filename, csv_path, ndjson_path,
                      errors_path, total_rows, succeeded_rows, failed_rows, error_message,
                      created_at, updated_at, started_at, finished_at
            "#,
        )
        .bind(new.account_id)
        .bind(new.user_id)
        .bind(&new.source_filename)
        .bind(&new.csv_path)
        .fetch_one(self.pool)
        .await?;
        Ok(row)
    }

    pub async fn get_by_id(&self, id: Uuid) -> RepoResult<ImportJob> {
        sqlx::query_as::<_, ImportJob>(
            r#"
            SELECT id, account_id, user_id, status, source_filename, csv_path, ndjson_path,
                   errors_path, total_rows, succeeded_rows, failed_rows, error_message,
                   created_at, updated_at, started_at, finished_at
            FROM import_jobs WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?
        .ok_or(RepoError::NotFound)
    }

    pub async fn update_status(
        &self,
        id: Uuid,
        status: ImportJobStatus,
        error_message: Option<&str>,
    ) -> RepoResult<ImportJob> {
        let row = sqlx::query_as::<_, ImportJob>(
            r#"
            UPDATE import_jobs SET
                status = $2,
                error_message = COALESCE($3, error_message),
                updated_at = now(),
                started_at = CASE
                    WHEN $2 IN ('converting', 'importing') AND started_at IS NULL THEN now()
                    ELSE started_at
                END,
                finished_at = CASE
                    WHEN $2 IN ('completed', 'failed') THEN now()
                    ELSE finished_at
                END
            WHERE id = $1
            RETURNING id, account_id, user_id, status, source_filename, csv_path, ndjson_path,
                      errors_path, total_rows, succeeded_rows, failed_rows, error_message,
                      created_at, updated_at, started_at, finished_at
            "#,
        )
        .bind(id)
        .bind(status.as_str())
        .bind(error_message)
        .fetch_optional(self.pool)
        .await?
        .ok_or(RepoError::NotFound)?;
        Ok(row)
    }

    pub async fn update_paths_and_counts(
        &self,
        id: Uuid,
        ndjson_path: Option<&str>,
        errors_path: Option<&str>,
        total_rows: i64,
        succeeded_rows: i64,
        failed_rows: i64,
    ) -> RepoResult<ImportJob> {
        let row = sqlx::query_as::<_, ImportJob>(
            r#"
            UPDATE import_jobs SET
                ndjson_path = COALESCE($2, ndjson_path),
                errors_path = COALESCE($3, errors_path),
                total_rows = $4,
                succeeded_rows = $5,
                failed_rows = $6,
                updated_at = now()
            WHERE id = $1
            RETURNING id, account_id, user_id, status, source_filename, csv_path, ndjson_path,
                      errors_path, total_rows, succeeded_rows, failed_rows, error_message,
                      created_at, updated_at, started_at, finished_at
            "#,
        )
        .bind(id)
        .bind(ndjson_path)
        .bind(errors_path)
        .bind(total_rows)
        .bind(succeeded_rows)
        .bind(failed_rows)
        .fetch_optional(self.pool)
        .await?
        .ok_or(RepoError::NotFound)?;
        Ok(row)
    }
}
