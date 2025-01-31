use crate::schema::session;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = session)]
pub struct Session {
    pub user_id: String,
    pub token: String,
    pub token_valid: bool,
}

#[derive(Serialize, Deserialize, Queryable)]
#[diesel(table_name = session)]

pub struct SessionData {
    pub id: i32,
    pub user_id: String,
    pub token: String,
    pub token_valid: bool,
}
