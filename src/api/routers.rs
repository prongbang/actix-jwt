use actix_web::web;
use crate::api;

pub fn initialize(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1")
        .configure(api::search::router::initialize)
        .configure(api::user::router::initialize)
    );
}