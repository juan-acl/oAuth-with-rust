use crate::handlers::user_handler::{
    create_user, delete_user, get_all_users, sign_in, update_user,
};
use actix_web::web;

pub fn main_routes_users(cfg: &mut web::ServiceConfig) {
    cfg.route("/users", web::get().to(get_all_users));
    cfg.route("/user", web::post().to(create_user));
    cfg.route("/user/signIn", web::post().to(sign_in));
    cfg.route("/user/{user_id}", web::delete().to(delete_user));
    cfg.route("/user", web::put().to(update_user));
}
