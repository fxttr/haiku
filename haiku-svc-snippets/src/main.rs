/*
 * Haiku
 *
 * Copyright (c) 2022. enso
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;
use diesel::r2d2::ConnectionManager;

mod controller;
mod data;
pub mod schema;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder().build(manager).expect(&format!("Error connecting to {}", database_url));

    // Start http server
    HttpServer::new(move || {
        App::new().service(web::scope("api/v1/snippets").data(pool.clone()).configure(services::snippets::configure))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
