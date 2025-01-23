use crate::handlers::user_handler::{create_user, get_all_users};
use actix_web::web;

pub fn main_routes_users(cfg: &mut web::ServiceConfig) {
    cfg.route("/users", web::get().to(get_all_users));
    cfg.route("/users", web::post().to(create_user));
}
