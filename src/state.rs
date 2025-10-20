use sqlx::PgPool;
use std::sync::Arc;

// Shared application state (like Go’s struct)
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}
