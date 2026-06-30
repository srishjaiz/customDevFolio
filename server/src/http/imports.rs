//! CSV upload routes (Phase 5). Phase 4 registers a no-op extension point.

use axum::Router;

use crate::http::AppState;

/// Register import routes on the API router (filled in Phase 5).
pub fn register(router: Router<std::sync::Arc<AppState>>) -> Router<std::sync::Arc<AppState>> {
    router
}
