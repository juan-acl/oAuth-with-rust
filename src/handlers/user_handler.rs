use crate::models::response_model::Response;
use actix_web::{web, Responder};

pub async fn get_all_users() -> impl Responder {
    web::Json(Response {
        status: "Ok".to_string(),
        message: "Hey there!".to_string(),
    })
}
