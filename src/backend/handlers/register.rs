use actix_web::web::ServiceConfig;
use crate::routes::version;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(version::version);
}