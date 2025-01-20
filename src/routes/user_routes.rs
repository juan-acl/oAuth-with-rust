use crate::handlers::user_handler::get_all_users;
use actix_web::web;

pub fn get_all_users_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/users", web::get().to(get_all_users));
}
