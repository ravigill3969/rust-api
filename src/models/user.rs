use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct VerifyUserResponse {
    pub success: bool,
    pub id: String,
    pub message: String,
}

#[derive(FromRow, Debug, Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
}

#[derive(FromRow, Debug)]
pub struct LoginDBResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}
