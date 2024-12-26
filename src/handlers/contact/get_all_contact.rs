use crate::utils::jwt::validate_token;
use actix_web::{HttpRequest, HttpResponse};
use serde_json::json;

// Protected endpoint for GET /contact
pub async fn get_all_contact(req: HttpRequest) -> HttpResponse {
    // Extract Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Strip "Bearer " prefix
                match validate_token(token) {
                    Ok(_) => {
                        // TODO: Fetch and return the contacts from the database
                        return HttpResponse::Ok().json(json!({
                            "message": "Access granted. Fetch contacts here."
                        }));
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
