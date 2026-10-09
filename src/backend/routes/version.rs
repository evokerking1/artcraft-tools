use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;
use crate::utils::env::get_backend_version;

#[derive(Serialize)]
struct Version {
    version: String,
}

#[get("/api/version")]
pub async fn version() -> impl Responder {
    let version = get_backend_version(); 
    HttpResponse::Ok().json(Version { version })
}
