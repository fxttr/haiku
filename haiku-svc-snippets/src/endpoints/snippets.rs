use actix_web::{get, post, web, HttpResponse};
use haiku_error::service::ServiceFailedError;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(list).service(create);
}

#[get("")]
async fn list() -> Result<HttpResponse, ServiceFailedError> {
    Ok(HttpResponse::Ok().body(format!("Hello!")))
}

#[post("")]
async fn create() -> Result<HttpResponse, ServiceFailedError> {
    Ok(HttpResponse::Ok().body(""))
}

