#[macro_use]
extern crate diesel;

use std::env;
use actix_web::{App, HttpServer, web};
use diesel::{PgConnection, Connection};
use dotenv::dotenv;

pub mod schema;
mod data;
mod controller;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    
    // Start http server
    HttpServer::new(move || {
        App::new().service(web::scope("api/v1/snippets").configure(services::snippets::configure))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
