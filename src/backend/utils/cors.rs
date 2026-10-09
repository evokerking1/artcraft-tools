use actix_cors::Cors;
use actix_web::http::header;
use crate::utils::env::get_cors_origin;

pub fn cors() -> Cors {
    let cors_origin = get_cors_origin();

    Cors::default()
        .allowed_origin(&cors_origin)
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .supports_credentials()
}
