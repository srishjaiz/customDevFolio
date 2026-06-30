use std::path::PathBuf;
use std::sync::Arc;

use crate::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    /// Root for uploaded CSV / NDJSON (`data/imports` by default).
    pub data_dir: PathBuf,
    /// Max upload size in bytes (Phase 5/7).
    pub max_upload_bytes: usize,
}

impl AppState {
    pub fn new(pool: PgPool, data_dir: PathBuf, max_upload_bytes: usize) -> Arc<Self> {
        Arc::new(Self {
            pool,
            data_dir,
            max_upload_bytes,
        })
    }
}
