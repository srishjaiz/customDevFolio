use thiserror::Error;

pub type RepoResult<T> = Result<T, RepoError>;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("not found")]
    NotFound,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("invalid input: {0}")]
    Invalid(String),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl RepoError {
    pub fn is_unique_violation(err: &sqlx::Error) -> bool {
        match err {
            sqlx::Error::Database(db) => db.code().as_deref() == Some("23505"),
            _ => false,
        }
    }
}
