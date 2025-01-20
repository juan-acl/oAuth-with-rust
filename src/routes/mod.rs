use actix_web::web;

pub mod user_routes;

pub fn get_all_users(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/"))
        .configure(user_routes::get_all_users_routes);
}
