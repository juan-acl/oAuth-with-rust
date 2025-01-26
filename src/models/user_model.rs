use crate::schema::user;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub address: String,
    pub phone_number: String,
    pub password: String,
    pub active: bool,
}

#[derive(Queryable, Serialize, Deserialize, Insertable, Validate)]
#[diesel(table_name = user)]
pub struct NewUser {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub lastname: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub address: String,
    #[validate(length(min = 1))]
    pub phone_number: String,
    #[validate(length(min = 1, message = "La contraseña no puede estar vacía"))]
    pub password: String,
}
