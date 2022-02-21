mod endpoints;

use actix_web::{App, HttpServer, get, web};
use actix_web::Responder;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new().service(web::scope("api/v1/snippets").configure(endpoints::snippets::configure))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
