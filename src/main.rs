use actix_web::{App, HttpServer};

mod config;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(|cfg| routes::get_all_users(cfg)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
