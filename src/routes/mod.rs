// src/routes/mod.rs

use crate::handlers::{auth, contact};
use actix_web::{web, HttpResponse};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index)) // Root route
        .route("/login", web::post().to(auth::login)) // Login route
        .route("/contact", web::get().to(contact::contact)); // Contact route
}

// Home route handler.
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Actix Web!")
}
