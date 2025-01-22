use crate::models::user_model::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub message: String,
    pub data: Option<Vec<User>>,
}
