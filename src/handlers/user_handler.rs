use crate::models::user_model::User;
use crate::schema::user::dsl::*;
use crate::{db::db::DbPool, models::response_model::Response};
use actix_web::{web, Responder};
use diesel::RunQueryDsl;

pub async fn get_all_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("No se puedo cargar la conexión");
    let results: Vec<User> = user
        .load(&mut connection)
        .expect("Error al cargar los usuarios");
    web::Json(Response {
        status: "Ok".to_string(),
        message: "Usuarios obtenidos satisfactoriamente".to_string(),
        data: Some(results),
    })
}
