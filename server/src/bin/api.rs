//! customfolio-api — free Axum HTTP server (Phases 4+).

use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

use customfolio_server::http::{router, AppState};
use customfolio_server::{connect, migrate};
use tower_http::cors::{Any, CorsLayer};
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
    let cors_origin = env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".into());
    let app = router(state).layer(TraceLayer::new_for_http()).layer(
        CorsLayer::new()
            .allow_origin(cors_origin.parse::<axum::http::HeaderValue>()?)
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_credentials(true),
    );

    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    tracing::info!("customfolio-api listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
