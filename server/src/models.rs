use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,
    pub display_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub password_hash: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub owner_user_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub default_theme: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewAccount {
    pub owner_user_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub default_theme: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Portfolio {
    pub id: Uuid,
    pub account_id: Uuid,
    pub slug: String,
    pub domain: String,
    pub person_name: String,
    pub config: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Insert or upsert a portfolio. `config` is the full PortfolioConfig JSON document.
#[derive(Debug, Clone)]
pub struct NewPortfolio {
    pub account_id: Uuid,
    pub slug: String,
    pub domain: String,
    pub person_name: String,
    pub config: JsonValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImportJobStatus {
    Pending,
    Converting,
    Importing,
    Completed,
    Failed,
}

impl ImportJobStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Converting => "converting",
            Self::Importing => "importing",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "converting" => Some(Self::Converting),
            "importing" => Some(Self::Importing),
            "completed" => Some(Self::Completed),
            "failed" => Some(Self::Failed),
            _ => None,
        }
    }
}

impl std::fmt::Display for ImportJobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ImportJob {
    pub id: Uuid,
    pub account_id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub source_filename: Option<String>,
    pub csv_path: Option<String>,
    pub ndjson_path: Option<String>,
    pub errors_path: Option<String>,
    pub total_rows: i64,
    pub succeeded_rows: i64,
    pub failed_rows: i64,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewImportJob {
    pub account_id: Uuid,
    pub user_id: Uuid,
    pub source_filename: Option<String>,
    pub csv_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewSession {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_job_status_roundtrip() {
        for s in [
            ImportJobStatus::Pending,
            ImportJobStatus::Converting,
            ImportJobStatus::Importing,
            ImportJobStatus::Completed,
            ImportJobStatus::Failed,
        ] {
            assert_eq!(ImportJobStatus::parse(s.as_str()), Some(s));
            assert_eq!(s.to_string(), s.as_str());
        }
        assert!(ImportJobStatus::parse("nope").is_none());
    }
}
