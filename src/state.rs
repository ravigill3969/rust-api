use std::sync::Arc;
use sqlx::PgPool;
use crate::utils::jwt::JwtManager;

#[derive(Clone)]
pub struct UserHandler {
  pub  db: Arc<PgPool>,
  pub  jwt: Arc<JwtManager>,
}

impl UserHandler {
    pub fn new(db: Arc<PgPool>, jwt: Arc<JwtManager>) -> Self {
        Self { db, jwt }
    }
}
