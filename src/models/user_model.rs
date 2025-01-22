use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub address: String,
    pub phone_number: String,
    pub password: String,
}
