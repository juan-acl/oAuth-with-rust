use crate::models::user_model::{NewUser, User};
use crate::schema::user::dsl::*;
use crate::{db::db::DbPool, models::response_model::ApiResponse};
use actix_web::{web, HttpResponse, Responder};
use diesel::result::Error as DieselError;
use diesel::RunQueryDsl;

pub async fn get_all_users(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get();
    if conn.is_err() {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "Error al obtener el pool de conexiones".to_string(),
        ));
    }

    let mut connection = conn.unwrap();

    let results: Result<Vec<User>, DieselError> = user.load(&mut connection);
    match results {
        Ok(users) => HttpResponse::Ok().json(ApiResponse {
            code: 200,
            message: String::from("Usuarios obtenidos exitosamente"),
            data: Some(users),
        }),
        Err(e) => {
            println!("Error al obtener los usuarios: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500,
                "Error al obtener los usuarios".to_string(),
            ))
        }
    }
}

pub async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> impl Responder {
    print!("Entro el el modulo de creacion de usuario");
    let conn = pool.get();
    if conn.is_err() {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "Error al obtener el pool de conexiones".to_string(),
        ));
    }

    let mut connection = conn.unwrap();

    let result = diesel::insert_into(user)
        .values(new_user.into_inner())
        .execute(&mut connection);

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success(
            200,
            "Usuario Creado satisfactoriamente".to_string(),
            (),
        )),
        Err(e) => {
            println!("Error al crear el usuario: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500,
                "Error al crear el usuario".to_string(),
            ))
        }
    }
}
