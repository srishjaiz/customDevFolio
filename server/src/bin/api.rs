//! customfolio-api — free Axum HTTP server (Phases 4+).
//!
//! Auth is **session cookies only** (`customfolio_session`, httpOnly).
//! CORS must use an explicit origin + header list when credentials are enabled —
//! `Allow-Headers: *` with `Allow-Credentials: true` is invalid and panics at boot.

use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use axum::http::{header, HeaderValue, Method};
use customfolio_server::http::{router, AppState};
use customfolio_server::{connect, migrate};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://customfolio:customfolio@localhost:5432/customfolio".into());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let data_dir = PathBuf::from(env::var("DATA_DIR").unwrap_or_else(|_| "data/imports".into()));
    let max_upload_bytes: usize = env::var("MAX_UPLOAD_BYTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(50 * 1024 * 1024);

    std::fs::create_dir_all(&data_dir)?;

    let pool = connect(&database_url).await?;
    migrate(&pool).await?;

    let state = AppState::new(pool, data_dir, max_upload_bytes);
    // Comma-separated list. Defaults cover common Next dev origins for cookie auth.
    let cors_origins_raw = env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3000,http://127.0.0.1:3000".into());
    let origin_headers: Vec<HeaderValue> = cors_origins_raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<HeaderValue>())
        .collect::<Result<Vec<_>, _>>()?;
    if origin_headers.is_empty() {
        anyhow::bail!("CORS_ORIGIN must list at least one browser origin");
    }
    // Credentials + wildcards are invalid; list methods/headers explicitly for cookie auth.
    let cors = CorsLayer::new()
        .allow_origin(origin_headers)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::AUTHORIZATION,
            header::COOKIE,
            header::HeaderName::from_static("x-requested-with"),
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    let app = router(state).layer(TraceLayer::new_for_http()).layer(cors);

    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    tracing::info!(
        "customfolio-api listening on http://{addr} (session-cookie auth; CORS origins={cors_origins_raw})"
    );
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
