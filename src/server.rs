use actix_web::{App, HttpServer, middleware};
use actix_web::dev::Server;
use actix_web::web::{Data, get};
use crate::api::routers;
use crate::api::search::model::SearchClient;
use crate::di;

pub fn create_server() -> Result<Server, std::io::Error> {
    let search = Data::new(SearchClient::new());

    log::info!("Starting HTTP server: go to http://localhost:8080");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&search))
            .app_data(Data::new(di::provider::inject()))
            .wrap(middleware::Logger::default())
            .route("/", get().to(|| async { "Hello Rust!" }))
            .configure(routers::initialize)
    })
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}