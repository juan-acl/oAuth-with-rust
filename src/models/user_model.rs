use crate::schema::user;
use diesel::prelude::{AsChangeset, Insertable, Queryable};
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
    #[validate(length(min = 1, message = "El nombre no puede estar vacío"))]
    pub name: String,
    #[validate(length(min = 1, message = "El apellido no puede estar vacío"))]
    pub lastname: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "La dirección no puede estar vacía"))]
    pub address: String,
    #[validate(length(min = 1, message = "El número de teléfono no puede estar vacío"))]
    pub phone_number: String,
    #[validate(length(min = 1, message = "La contraseña no puede estar vacía"))]
    pub password: String,
}

#[derive(Deserialize, Validate, Queryable, Serialize, AsChangeset)]
#[diesel(table_name = user)]
pub struct UpdateUser {
    #[validate(range(min = 1))]
    pub id: i32,
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(length(min = 1))]
    pub lastname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 1))]
    pub address: Option<String>,
    #[validate(length(min = 1))]
    pub phone_number: Option<String>,
    #[validate(length(min = 1))]
    pub password: Option<String>,
    pub active: Option<bool>,
}

#[derive(Deserialize, Validate, Serialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}
