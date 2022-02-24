use actix_web::{App, HttpServer, web};
use actix_web::Responder;

mod data;
mod controller;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new().service(web::scope("api/v1/snippets").configure(services::snippets::configure))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
