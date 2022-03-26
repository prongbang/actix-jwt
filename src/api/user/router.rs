use actix_web::{get, web, Responder};
use crate::{middleware};
use crate::api::user::get_user_me_usecase;
use crate::di::provider::Container;

#[get("/user/me")]
async fn get_user_me(_: middleware::jwt::Authorization, container: web::Data<Container>) -> impl Responder {
    let result = get_user_me_usecase::execute(&container.user_repo).await;
    format!("I am {}", result)
}

pub fn initialize(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_me);
}