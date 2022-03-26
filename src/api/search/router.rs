use actix_web::{get, web, Responder};
use actix_web::web::Data;
use crate::api::search::model::{Search, SearchClient};

#[get("/search")]
async fn get_search(search: Data<SearchClient>) -> impl Responder {
    let result = search.search("Devไปวันๆ".to_string());
    format!("{}", result)
}

pub fn initialize(cfg: &mut web::ServiceConfig) {
    cfg.service(get_search);
}