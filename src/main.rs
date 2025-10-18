mod models;
mod routes;
mod utils;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running on http://127.0.0.1:8080");

    HttpServer::new(|| App::new().configure(routes::init))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
