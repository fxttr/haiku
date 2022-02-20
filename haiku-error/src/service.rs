use core::fmt;
use std::error::Error;
use std::fmt::{Display, Result, Debug, Formatter};
use actix_web::Error as ActixError;
use actix_web::{ResponseError, http::StatusCode, HttpResponse};

#[derive(Error, Debug)]
pub enum ServiceErrorType {
    #[error("Internal server error")]
    Internal
}

pub struct ServiceFailedError {
    error_type: ServiceErrorType 
}

impl From<ServiceErrorType> for ActixError {
    fn from(error_type: ServiceErrorType) -> Self {
	error_type.into()
    }
}

impl Error for ServiceFailedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
	self.error_type.source()
    }
}

impl Display for ServiceFailedError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
	self.error_type.fmt(formatter)
    }
}

impl Debug for ServiceFailedError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
	formatter.debug_struct(&format!("ServiceFailedError: {:?}", &self.error_type)).field("error_type", &self.error_type).finish()
    }
}

impl ResponseError for ServiceFailedError {
    fn status_code(&self) -> StatusCode {
    }

    fn error_response(&self) -> HttpResponse {
	
    }
}
