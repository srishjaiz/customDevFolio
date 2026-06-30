//! Manual CSV upload → NDJSON on disk → Postgres (Phase 5).

use std::sync::Arc;

use axum::extract::{Multipart, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use uuid::Uuid;

use crate::http::routes::{api_err, require_user};
use crate::http::AppState;
use crate::import::import_ndjson_file;
use crate::models::{ImportJobStatus, NewImportJob};
use crate::repos::{AccountRepo, ImportJobRepo};

pub fn register(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route("/accounts/{account_id}/imports", post(create_import))
        .route("/imports/{job_id}", get(get_import))
}

async fn create_import(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(account_id): Path<Uuid>,
    mut multipart: Multipart,
) -> Response {
    let user = match require_user(&state, &headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    let account = match AccountRepo::new(&state.pool).get_by_id(account_id).await {
        Ok(a) => a,
        Err(_) => return api_err(StatusCode::NOT_FOUND, "account not found"),
    };
    if account.owner_user_id != user.id {
        return api_err(StatusCode::FORBIDDEN, "not account owner");
    }

    let mut csv_bytes: Option<Vec<u8>> = None;
    let mut filename = "upload.csv".to_string();
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" || name == "csv" {
            if let Some(fn_) = field.file_name() {
                filename = fn_.to_string();
            }
            match field.bytes().await {
                Ok(b) => {
                    if b.len() > state.max_upload_bytes {
                        return api_err(
                            StatusCode::PAYLOAD_TOO_LARGE,
                            format!("file exceeds max {} bytes", state.max_upload_bytes),
                        );
                    }
                    csv_bytes = Some(b.to_vec());
                }
                Err(e) => return api_err(StatusCode::BAD_REQUEST, e.to_string()),
            }
        }
    }
    let Some(csv_bytes) = csv_bytes else {
        return api_err(StatusCode::BAD_REQUEST, "missing multipart field file/csv");
    };

    let job = match ImportJobRepo::new(&state.pool)
        .create(&NewImportJob {
            account_id,
            user_id: user.id,
            source_filename: Some(filename),
            csv_path: None,
        })
        .await
    {
        Ok(j) => j,
        Err(e) => return api_err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    let work_dir = state.data_dir.join(job.id.to_string());
    if let Err(e) = tokio::fs::create_dir_all(&work_dir).await {
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
    }
    let csv_path = work_dir.join("source.csv");
    let ndjson_path = work_dir.join("data.ndjson");
    let errors_path = work_dir.join("errors.ndjson");
    if let Err(e) = tokio::fs::write(&csv_path, &csv_bytes).await {
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
    }

    let _ = ImportJobRepo::new(&state.pool)
        .update_status(job.id, ImportJobStatus::Converting, None)
        .await;

    let conv = tokio::task::spawn_blocking({
        let csv_path = csv_path.clone();
        let ndjson_path = ndjson_path.clone();
        let errors_path = errors_path.clone();
        move || crate::csv_convert::csv_file_to_ndjson(&csv_path, &ndjson_path, Some(&errors_path))
    })
    .await;

    match conv {
        Ok(Ok(_)) => {}
        Ok(Err(e)) => {
            let _ = ImportJobRepo::new(&state.pool)
                .update_status(job.id, ImportJobStatus::Failed, Some(&e.to_string()))
                .await;
            return api_err(StatusCode::BAD_REQUEST, e.to_string());
        }
        Err(e) => return api_err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }

    let stats = match import_ndjson_file(
        &state.pool,
        account_id,
        &ndjson_path,
        Some(&errors_path),
        true,
        Some(job.id),
    )
    .await
    {
        Ok(s) => s,
        Err(e) => {
            let _ = ImportJobRepo::new(&state.pool)
                .update_status(job.id, ImportJobStatus::Failed, Some(&e.to_string()))
                .await;
            return api_err(StatusCode::BAD_REQUEST, e.to_string());
        }
    };

    let job = ImportJobRepo::new(&state.pool)
        .get_by_id(job.id)
        .await
        .unwrap_or(job);

    Json(serde_json::json!({
        "job": job,
        "stats": {
            "total_rows": stats.total_rows,
            "succeeded": stats.succeeded,
            "failed": stats.failed,
        }
    }))
    .into_response()
}

async fn get_import(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(job_id): Path<Uuid>,
) -> Response {
    let user = match require_user(&state, &headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    let job = match ImportJobRepo::new(&state.pool).get_by_id(job_id).await {
        Ok(j) => j,
        Err(_) => return api_err(StatusCode::NOT_FOUND, "job not found"),
    };
    let account = match AccountRepo::new(&state.pool).get_by_id(job.account_id).await {
        Ok(a) => a,
        Err(_) => return api_err(StatusCode::NOT_FOUND, "account not found"),
    };
    if account.owner_user_id != user.id {
        return api_err(StatusCode::FORBIDDEN, "not account owner");
    }
    Json(job).into_response()
}
