use actix_web::{delete, get, HttpResponse, post, put, web};

use haiku_error::service::ServiceFailedError;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(list)
        .service(create)
        .service(delete)
        .service(update);
}

#[get("")]
async fn list() -> Result<HttpResponse, ServiceFailedError> {
    Ok(HttpResponse::Ok().body(format!("Hello!")))
}

#[post("")]
async fn create() -> Result<HttpResponse, ServiceFailedError> {
    todo!()
}

#[delete("")]
async fn delete() -> Result<HttpResponse, ServiceFailedError> {
    todo!()
}

#[put("")]
async fn update() -> Result<HttpResponse, ServiceFailedError> {
    todo!()
}