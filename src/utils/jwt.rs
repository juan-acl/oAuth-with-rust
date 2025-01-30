use std::{env, time::Duration};

use crate::models::jwt_model::Claims;
use actix_web::{web::Json, HttpResponse};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::schema::user;

async fn create_token(time_expiration_min: i64, user: User) -> String {
    let header = Header::new(Algorithm::HS512);
    let encoding_key = EncodingKey::from_secret(env::var("KEY_JWT".as_ref()));

    let expiration = (Utc::now() + Duration::minutes(time_expiration_min)).timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;

    let my_claims = Claims {
        exp: expiration as usize,
        iat: iat as usize,
        user,
    };

    encode(&header, &my_claims, &encoding_key);
}

async fn get_token_header(Json(user): Json<User>) -> HttpResponse {
    let token = jwt_lib::get_jwt(user);

    match token {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
