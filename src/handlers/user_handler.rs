use crate::models::session::{Session, SessionData};
use crate::models::user_model::{Login, NewUser, UpdateUser, User};
use crate::schema::session::dsl::session;
use crate::schema::session::{token_valid, user_id as user_id_session};
use crate::schema::user::dsl::*;
use crate::utils::bcrypt::{hash_password, verify_password};
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

    let results: Result<Vec<User>, DieselError> =
        user.filter(active.eq(true)).load(&mut connection);
    match results {
        Ok(users) => HttpResponse::Ok().json(ApiResponse {
            code: 200,
            message: String::from("Usuarios obtenidos exitosamente"),
            data: Some(users),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "Error al obtener los usuarios".to_string(),
        )),
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

    let user_exist: Result<Vec<User>, DieselError> =
        user.filter(email.eq(&new_user.email)).load(&mut connection);

    if user_exist.is_ok() && user_exist.unwrap().len() > 0 {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            400,
            "El usuario ya existe".to_string(),
        ));
    }

    let user_insert = NewUser {
        password: hash_password(&new_user.password),
        ..new_user.into_inner()
    };

    let result = diesel::insert_into(user)
        .values(&user_insert)
        .execute(&mut connection);

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success(
            200,
            "Usuario creado satisfactoriamente".to_string(),
            (),
        )),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            500,
            "Error al crear el usuario".to_string(),
        )),
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

    let find_user = user
        .filter(email.eq(&user_login.email))
        .first::<User>(&mut connection);

    if find_user.is_err() {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(
            404,
            "Usuario no encontrado".to_string(),
        ));
    }

    let validate_password = verify_password(&user_login.password, &find_user.unwrap().password);

    if !validate_password {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            400,
            "Credenciales incorrectas.".to_string(),
        ));
    }

    let session_exist = session
        .filter(user_id_session.eq(&user_login.email))
        .first::<SessionData>(&mut connection);

    if session_exist.is_ok() {
        let update_token_session = diesel::update(
            session
                .filter(user_id_session.eq(&user_login.email))
                .filter(token_valid.eq(true)),
        )
        .set(token_valid.eq(false))
        .execute(&mut connection);

        match update_token_session {
            Ok(_) => (),
            Err(e) => {
                println!("Error al actualizar la sesión: {:?}", e);
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    500,
                    "Error al actualizar la sesión".to_string(),
                ));
            }
        }
    }

    let user_login_clone = Login {
        email: user_login.email.clone(),
        password: user_login.password.clone(),
    };

    let token_created = create_token_session(60, user_login_clone);

    let insert_session = Session {
        user_id: user_login.email.clone(),
        token: token_created.clone(),
        token_valid: true,
    };

    let result_session = diesel::insert_into(session)
        .values(&insert_session)
        .execute(&mut connection);

    match result_session {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            code: 200,
            message: "Sesión creada exitosamente".to_string(),
            data: Some(token_created),
        }),
        Err(e) => {
            println!("Error al crear la sesión: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500,
                "Error al crear la sesión".to_string(),
            ))
        }
    }
}
