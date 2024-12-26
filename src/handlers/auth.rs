use crate::config::get_env;
use crate::models::user::User;
use crate::utils::{hash::verify_password, jwt::generate_token};
use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;

pub async fn login(db_pool: web::Data<PgPool>, login_data: web::Json<User>) -> HttpResponse {
    let username = &login_data.username;
    let password = &login_data.password;

    let query = "SELECT password FROM users WHERE username = $1";
    let row = sqlx::query_scalar::<_, String>(query)
        .bind(username)
        .fetch_optional(db_pool.get_ref())
        .await;

    match row {
        Ok(Some(hashed_password)) if verify_password(&hashed_password, password) => {
            let token = generate_token(username);
            HttpResponse::Ok().json(json!({ "token": token }))
        }
        _ => HttpResponse::Unauthorized().json(json!({ "error": "Invalid credentials" })),
    }
}

pub async fn seed_admin(db_pool: &PgPool) {
    let username = get_env("ADMIN_USERNAME");
    let password = get_env("ADMIN_PASSWORD");

    let existing_user: Option<(i32,)> = sqlx::query_as("SELECT id FROM users WHERE username = $1")
        .bind(&username)
        .fetch_optional(db_pool)
        .await
        .expect("Failed to check existing admin");

    if existing_user.is_none() {
        let hashed_password = crate::utils::hash::hash_password(&password);

        sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
            .bind(&username)
            .bind(&hashed_password)
            .execute(db_pool)
            .await
            .expect("Failed to seed admin user");

        println!("Admin user seeded: {}", username);
    } else {
        println!("Admin user already exists: {}", username);
    }
}
