use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}
