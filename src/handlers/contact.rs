use crate::utils::jwt::validate_token;
use actix_web::{HttpRequest, HttpResponse};
use serde_json::json;

pub async fn contact(req: HttpRequest) -> HttpResponse {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(token) = header_value.to_str() {
            if validate_token(token).is_ok() {
                return HttpResponse::Ok().json(json!({
                    "message": "Welcome to contacts"
                }));
            } else {
                return HttpResponse::Unauthorized().json(json!({
                    "error": "Invalid token"
                }));
            }
        }
    }

    HttpResponse::Unauthorized().json(json!({
        "error": "Token is missing"
    }))
}
