use crate::models::user::{
    LoginDBResponse, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, User,
};
use crate::state::UserHandler;
use crate::utils::password::{hash_password, verify_password};
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[post("/api/register")]
pub async fn register(
    handler: web::Data<UserHandler>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let pool: &PgPool = &handler.db;

    let hashed = match hash_password(&req.password) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Password hashing failed"),
    };

    let query = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING username, email
        "#,
        req.username,
        req.email,
        hashed,
    )
    .fetch_one(pool)
    .await;

    match query {
        Ok(user) => HttpResponse::Ok().json(RegisterResponse {
            success: true,
            message: format!("User '{}' ({}) created", user.username, user.email),
        }),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("DB insert failed")
        }
    }
}

#[post("/api/login")]
pub async fn login(
    handler: web::Data<UserHandler>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let pool: &PgPool = &handler.db;

    let result = sqlx::query_as!(
        LoginDBResponse,
        r#"
        SELECT id, username, password_hash
        FROM users
        WHERE email = $1
        "#,
        req.email
    )
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some(user)) => {
            if verify_password(&req.password, &user.password_hash) {
                let token = handler
                    .jwt
                    .generate(&user.id.to_string(), &user.email)
                    .unwrap();

                HttpResponse::Ok().json(LoginResponse {
                    success: true,
                    message: format!("Welcome back, {}! Token: {}", user.username, token),
                })
            } else {
                HttpResponse::Unauthorized().json(LoginResponse {
                    success: false,
                    message: "Invalid email or password".to_string(),
                })
            }
        }
        Ok(None) => HttpResponse::NotFound().json(LoginResponse {
            success: false,
            message: "User not found".to_string(),
        }),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().json(LoginResponse {
                success: false,
                message: "Database error".to_string(),
            })
        }
    }
}
