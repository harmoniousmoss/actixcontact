use actix_web::{web, HttpResponse};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ContactRequest {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub message: String,
}

pub async fn post_contact(
    db_pool: web::Data<PgPool>,
    contact_data: web::Json<ContactRequest>,
) -> HttpResponse {
    // Validate required fields
    if contact_data.name.is_empty()
        || contact_data.email.is_empty()
        || contact_data.message.is_empty()
    {
        return HttpResponse::BadRequest().json(json!({
            "error": "Name, email, and message are required fields."
        }));
    }

    // Validate email format
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !email_regex.is_match(&contact_data.email) {
        return HttpResponse::BadRequest().json(json!({
            "error": "Invalid email format."
        }));
    }

    // Validate phone number if provided
    if let Some(phone) = &contact_data.phone {
        if !phone.chars().all(char::is_numeric) {
            return HttpResponse::BadRequest().json(json!({
                "error": "Phone number must contain only numeric characters."
            }));
        }
    }

    // Insert data into the database
    let query = r#"
        INSERT INTO contacts (id, name, email, phone, message, created_at)
        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)
    "#;

    let id = Uuid::new_v4();

    if let Err(err) = sqlx::query(query)
        .bind(id)
        .bind(&contact_data.name)
        .bind(&contact_data.email)
        .bind(&contact_data.phone)
        .bind(&contact_data.message)
        .execute(db_pool.get_ref())
        .await
    {
        return HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to submit contact form: {}", err)
        }));
    }

    // Respond with success
    HttpResponse::Ok().json(json!({
        "message": "Contact form submitted successfully.",
        "data": {
            "id": id,
            "name": contact_data.name,
            "email": contact_data.email,
            "phone": contact_data.phone,
            "message": contact_data.message,
        }
    }))
}
