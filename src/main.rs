mod models;
mod routes;
mod utils;
mod state;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use actix_web::{web, App, HttpServer};
use std::env;
use std::sync::Arc;
use crate::state::UserHandler;
use crate::utils::jwt::JwtManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running on http://127.0.0.1:8080");

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to postgres");

    println!("✅ Connected to Postgres at {}", db_url);

    let db = Arc::new(pool);
    let jwt = Arc::new(JwtManager::new(jwt_secret));
    let user_handler = UserHandler::new(db.clone(), jwt.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_handler.clone())) 
            .configure(routes::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
