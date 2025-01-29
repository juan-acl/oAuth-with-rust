use actix_web::{web::Json, HttpResponse};

use crate::schema::user;

async fn get_token_header(Json(user): Json<User>) -> HttpResponse {
    let token = jwt_lib::get_jwt(user);

    match token {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
