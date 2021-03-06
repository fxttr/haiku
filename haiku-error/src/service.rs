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

use std::collections::HashMap;
use std::fmt::Debug;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServiceErrorType {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Too Many Requests")]
    TooManyRequests,
}

#[derive(Debug, Display, Error)]
pub struct ServiceFailedError {
    #[display(fmt = "Error")]
    error_type: ServiceErrorType,
}

impl ServiceFailedError {
    pub fn error_type(&self) -> &ServiceErrorType {
        &self.error_type
    }

    pub fn internal_server_error() -> Self {
        ServiceErrorType::InternalServerError.into()
    }

    pub fn too_many_requests() -> Self {
        ServiceErrorType::TooManyRequests.into()
    }
}

impl From<ServiceErrorType> for actix_web::Error {
    fn from(error_type: ServiceErrorType) -> Self {
        let error: ServiceErrorType = error_type.into();
        error.into()
    }
}

impl<T> From<T> for ServiceFailedError
    where
        ServiceErrorType: From<T>,
{
    fn from(item: T) -> Self {
        ServiceFailedError {
            error_type: ServiceErrorType::from(item),
        }
    }
}

impl ResponseError for ServiceFailedError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            ServiceErrorType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceErrorType::TooManyRequests => StatusCode::TOO_MANY_REQUESTS
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut result = HashMap::new();
        result.insert("error".to_owned(), format!("{}", self.error_type));

        HttpResponse::InternalServerError().json(result)
    }
}
