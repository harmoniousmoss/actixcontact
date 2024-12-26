mod config;
mod handlers;
mod models;
mod utils;

use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::{Executor, PgPool};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Actix Web!")
}

async fn initialize_database(db_pool: &PgPool) {
    let create_table_query = r#"
    CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        username VARCHAR(255) UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL
    );
    "#;

    db_pool
        .execute(create_table_query)
        .await
        .expect("Failed to initialize database");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();

    let database_url = config::get_env("DATABASE_URL");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    initialize_database(&db_pool).await;

    handlers::auth::seed_admin(&db_pool).await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .route("/", web::get().to(index))
            .route("/login", web::post().to(handlers::auth::login))
            .route("/contact", web::get().to(handlers::contact::contact))
    })
    .bind(("127.0.0.1", 8080))?;

    let addr = server.addrs();
    for a in addr {
        println!("Server running at http://{}", a);
    }

    server.run().await
}
