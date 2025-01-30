use crate::models::user_model::{Login, NewUser, UpdateUser, User};
use crate::schema::user::dsl::*;
use crate::utils::jwt::create_token_session;
use crate::utils::validator::validate_and_extract_errors;
use crate::{db::db::DbPool, models::response_model::ApiResponse};
use actix_web::{web, HttpResponse, Responder};
use diesel::query_dsl::methods::FilterDsl;
use diesel::result::Error as DieselError;
use diesel::{ExpressionMethods, RunQueryDsl};

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
    if let Err(errors) = validate_and_extract_errors(&*new_user) {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, errors.join(", ")));
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

pub async fn update_user(
    pool: web::Data<DbPool>,
    user_update: web::Json<UpdateUser>,
) -> impl Responder {
    if let Err(errors) = validate_and_extract_errors(&*user_update) {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, errors.join(", ")));
    }
    let conn = pool.get();
    if conn.is_err() {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "Error al obtener el pool de conexion".to_string(),
        ));
    }

    let mut connection = conn.unwrap();

    let user_exist = user
        .filter(id.eq(user_update.id))
        .first::<User>(&mut connection);

    if let Err(diesel::result::Error::NotFound) = user_exist {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(
            404,
            "Usuario no encontrado".to_string(),
        ));
    }

    if user_exist.is_err() {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "Error al buscar el usuario".to_string(),
        ));
    }

    let update_data = UpdateUser {
        id: user_update.id,
        name: user_update.name.clone(),
        lastname: user_update.lastname.clone(),
        email: user_update.email.clone(),
        address: user_update.address.clone(),
        phone_number: user_update.phone_number.clone(),
        password: user_update.password.clone(),
        active: user_update.active,
    };

    let result = diesel::update(user.filter(id.eq(user_update.id)))
        .set(&update_data)
        .execute(&mut connection);

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success(
            200,
            "Usuario actualizado satisfactoriamente".to_string(),
            (),
        )),
        Err(e) => {
            println!("Error al actualizar el usuario: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500,
                "Error al actualizar el usuario".to_string(),
            ))
        }
    }
}

pub async fn sign_in(pool: web::Data<DbPool>, user_login: web::Json<Login>) -> impl Responder {
    print!("{:?}", user_login);
    if let Err(errors) = validate_and_extract_errors(&*user_login) {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, errors.join(", ")));
    }

    let conn = pool.get();

    if conn.is_err() {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "Error al obtener el pool de conexiones".to_string(),
        ));
    }

    let mut connection = conn.unwrap();
    let user_login_clone = Login {
        email: user_login.email.clone(),
        password: user_login.password.clone(),
    };

    let token = create_token_session(60, user_login_clone);
    return HttpResponse::Ok().json(ApiResponse {
        code: 200,
        message: "Usuario autenticado exitosamente".to_string(),
        data: Some(token),
    });
}
