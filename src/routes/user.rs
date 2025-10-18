use crate::models::user::{RegisterRequest, RegisterResponse};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/api/register")]
pub async fn register(req: web::Json<RegisterRequest>) -> impl Responder {
    let res = RegisterResponse {
        success: true,
        message: format!(
            "User '{}' with email '{}' registered successfully!",
            req.username, req.email
        ),
    };

    HttpResponse::Ok().json(res)
}
