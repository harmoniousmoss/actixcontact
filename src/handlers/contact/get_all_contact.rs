use crate::utils::jwt::validate_token;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use sqlx::PgPool;

// Protected endpoint for GET /contact
pub async fn get_all_contact(req: HttpRequest, db_pool: web::Data<PgPool>) -> HttpResponse {
    // Extract Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Strip "Bearer " prefix
                match validate_token(token) {
                    Ok(_) => {
                        // Query contacts from the database
                        match sqlx::query_as!(
                            Contact,
                            r#"
                            SELECT name, email, phone, message
                            FROM contacts
                            "#
                        )
                        .fetch_all(db_pool.get_ref())
                        .await
                        {
                            Ok(contacts) => {
                                return HttpResponse::Ok().json(json!({ "contacts": contacts }));
                            }
                            Err(err) => {
                                return HttpResponse::InternalServerError().json(json!({
                                    "error": "Failed to fetch contacts",
                                    "details": err.to_string()
                                }));
                            }
                        }
                    }
                    Err(_) => {
                        return HttpResponse::Unauthorized().json(json!({
                            "error": "Invalid token"
                        }));
                    }
                }
            }
        }
    }

    HttpResponse::Unauthorized().json(json!({
        "error": "Authorization token missing or invalid"
    }))
}

// Define the Contact struct for deserialization
#[derive(sqlx::FromRow, serde::Serialize)]
struct Contact {
    name: String,
    email: String,
    phone: Option<String>,
    message: String,
}
