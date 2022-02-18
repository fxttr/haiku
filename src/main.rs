use actix_web::{web, App, HttpServer};
use actix_web::Responder;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/users", web::get().to(get_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub async fn get_users() -> impl Responder {
    format!("hello from get users")
}
