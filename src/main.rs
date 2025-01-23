#![allow(dead_code)]
use actix_web::{web, App, HttpServer};
use std::env;

mod db;
mod handlers;
#[allow(dead_code)]
mod models;
mod routes;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::db::establish_connection();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::user_routes::main_routes_users)
    })
    .bind((
        env::var("BASE_URL").expect("BASE_URL debe estar configurada en .env"),
        env::var("PORT")
            .expect("PORT debe estar configurado en .env")
            .parse()
            .unwrap(),
    ))?
    .run()
    .await
}
