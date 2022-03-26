use actix_jwt::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::create_server().unwrap().await
}