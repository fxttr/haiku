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

use actix_web::{delete, get, HttpResponse, post, put, web};
use actix_web::web::Data;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use haiku_error::service::ServiceFailedError;
use crate::controller::snippets::SnippetsController;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(list)
        .service(create)
        .service(delete)
        .service(update);
}

#[get("")]
async fn list(data: Data<Pool<ConnectionManager<PgConnection>>>) -> Result<HttpResponse, ServiceFailedError> {
    let connection = data.get().expect("Couldn't get a database connection from pool.");
    let controller = SnippetsController::new(connection);

    match controller.find() {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(ServiceFailedError::internal_server_error())
    }
}

#[post("")]
async fn create(data: Data<Pool<ConnectionManager<PgConnection>>>) -> Result<HttpResponse, ServiceFailedError> {
    todo!()
}

#[delete("")]
async fn delete(data: Data<Pool<ConnectionManager<PgConnection>>>) -> Result<HttpResponse, ServiceFailedError> {
    todo!()
}

#[put("")]
async fn update(data: Data<Pool<ConnectionManager<PgConnection>>>) -> Result<HttpResponse, ServiceFailedError> {
    todo!()
}