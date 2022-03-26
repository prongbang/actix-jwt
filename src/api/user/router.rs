use actix_web::{get, web, Responder};
use crate::{middleware};

#[get("/user/me")]
async fn get_user_me(_: middleware::jwt::Authorization) -> impl Responder {
    format!("I am {}", "Devไปวันๆ")
}

pub fn initialize(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_me);
}