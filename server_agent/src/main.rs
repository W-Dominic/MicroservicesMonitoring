use actix_web::{get, web, Responder, Result, HttpResponse, HttpServer};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/auth")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 1337))?
        .run()
        .await
}