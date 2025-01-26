use crate::models::user_model::{NewUser, User};
use crate::schema::user::dsl::*;
use crate::{db::db::DbPool, models::response_model::ApiResponse};
use actix_web::{web, HttpResponse, Responder};
use diesel::query_dsl::methods::FilterDsl;
use diesel::result::Error as DieselError;
use diesel::{ExpressionMethods, RunQueryDsl};
use validator::Validate;

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
    if let Err(errors) = new_user.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    format!(
                        "{}: {}",
                        field,
                        error
                            .message
                            .clone()
                            .unwrap_or_else(|| "Error desconocido".to_string().into())
                    )
                })
            })
            .collect();

        return HttpResponse::BadRequest()
            .json(ApiResponse::<()>::error(400, error_messages.join(", ")));
    }

    let conn = pool.get();
    if conn.is_err() {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "Error al obtener el pool de conexiones".to_string(),
        ));
    }

    let mut connection = conn.unwrap();

    let user_exist: Result<Vec<User>, DieselError> = user
        .filter(email.eq(new_user.email.clone()))
        .load(&mut connection);

    if user_exist.is_ok() && user_exist.unwrap().len() > 0 {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            400,
            "El usuario ya existe".to_string(),
        ));
    }

    let result = diesel::insert_into(user)
        .values(new_user.into_inner())
        .execute(&mut connection);

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success(
            200,
            "Usuario creado satisfactoriamente".to_string(),
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

pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    let conn = pool.get();
    if conn.is_err() {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "Error al obtener el pool de conexiones".to_string(),
        ));
    }

    let mut connection = conn.unwrap();

    let user_exist = user.filter(id.eq(*user_id)).first::<User>(&mut connection);

    if let Err(diesel::result::Error::NotFound) = user_exist {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(
            404,
            "Usuario no encontrado".to_string(),
        ));
    } else if user_exist.is_err() {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "Error al buscar el usuario".to_string(),
        ));
    }

    let result = diesel::update(user.filter(id.eq(*user_id)))
        .set(active.eq(false))
        .execute(&mut connection);

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success(
            200,
            "Usuario eliminado satisfactoriamente".to_string(),
            (),
        )),
        Err(e) => {
            println!("Error al eliminar el usuario: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500,
                "Error al eliminar el usuario".to_string(),
            ))
        }
    }
}
