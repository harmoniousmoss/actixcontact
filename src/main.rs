use actix_web::{web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Actix Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?;

    let addr = server.addrs(); // Use `addrs` to get bound addresses
    for a in addr {
        println!("Server running at http://{}", a);
    }

    server.run().await
}
