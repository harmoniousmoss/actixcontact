mod config;
mod handlers;
mod init;
mod models;
mod routes;
mod utils;

use actix_web::App;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables
    config::init();

    // Establish database connection
    let database_url = config::get_env("DATABASE_URL");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");
    println!("✅ Connected to the database at {}", database_url);

    // Initialize database schema
    init::initialize_database(&db_pool).await;

    // Seed admin user
    match init::seed_admin(&db_pool).await {
        Ok(message) => println!("{}", message),
        Err(err) => println!("❌ Failed to seed admin user: {}", err),
    }

    // Start HTTP server
    let port = 8080;
    println!("🚀 Starting server at http://127.0.0.1:{}", port);

    actix_web::HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .configure(routes::configure_routes) // Centralized route configuration
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
