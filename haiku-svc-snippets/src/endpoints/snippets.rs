use actix_web::{HttpResponse, web, get};
use haiku_error::service::ServiceFailedError;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(list);
}

#[get("")]
async fn list() -> Result<HttpResponse, ServiceFailedError> {
    Ok(HttpResponse::Ok().body(format!("Hello!")))
}
