use actix_web::{get, web, Responder, Result, HttpResponse, HttpServer};

mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(move || {
        App::new()
        .service(auth::get_auth)
    })
    .bind(("127.0.0.1", 1337))?
    .run()
    .await
}