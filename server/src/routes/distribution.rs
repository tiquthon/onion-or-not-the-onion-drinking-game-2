use actix_web::{web, Responder};

#[tracing::instrument(name = "Distribution")]
pub async fn distribution() -> impl Responder {
    web::Json(crate::data::distribution())
}
