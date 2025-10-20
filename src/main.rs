mod models;
mod routes;
mod utils;
mod state;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use actix_web::{web, App, HttpServer};
use std::env;
use std::sync::Arc;
use crate::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running on http://127.0.0.1:8080");

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to postgres");

    println!("✅ Connected to Postgres at {}", db_url);

    // ✅ Wrap your pool inside AppState
    let app_state = AppState {
        db: Arc::new(pool),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone())) 
            .configure(routes::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
