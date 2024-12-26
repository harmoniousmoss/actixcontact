use crate::handlers::{auth, contact};
use actix_web::{web, HttpResponse};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index)) // Root route
        .route("/login", web::post().to(auth::login)) // Login route
        .route("/contact", web::post().to(contact::post_contact)) // Public POST /contact
        .route("/contact", web::get().to(contact::get_all_contact)); // Protected GET /contact
}

// Home route handler.
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Actix Web!")
}
