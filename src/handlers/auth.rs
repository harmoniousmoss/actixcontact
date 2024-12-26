// src/handlers/auth.rs
use crate::utils::{hash::verify_password, jwt::generate_token};
use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

// Struct for login request payload
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(
    db_pool: web::Data<PgPool>,
    login_data: web::Json<LoginRequest>,
) -> HttpResponse {
    let username = &login_data.username;
    let password = &login_data.password;

    let query = r#"
        SELECT id, password, created_at, updated_at
        FROM users
        WHERE username = $1
    "#;

    let row = sqlx::query_as::<_, (Uuid, String, NaiveDateTime, NaiveDateTime)>(query)
        .bind(username)
        .fetch_optional(db_pool.get_ref())
        .await;

    match row {
        Ok(Some((user_id, hashed_password, created_at, updated_at)))
            if verify_password(&hashed_password, password) =>
        {
            let token = generate_token(username);
            HttpResponse::Ok().json(json!({
                "id": user_id,
                "username": username,
                "token": token,
                "created_at": created_at,
                "updated_at": updated_at
            }))
        }
        _ => HttpResponse::Unauthorized().json(json!({ "error": "Invalid credentials" })),
    }
}
