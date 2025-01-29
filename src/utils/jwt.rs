use actix_web::{web::Json, HttpResponse};
use jsonwebtoken::{Algorithm, Header};

use crate::schema::user;

async fn create_token(time_expiration_min: i64, user: User) -> String {
    let header = Header::new(Algorithm::HS512);
}

async fn get_token_header(Json(user): Json<User>) -> HttpResponse {
    let token = jwt_lib::get_jwt(user);

    match token {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
