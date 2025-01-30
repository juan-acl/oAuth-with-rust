use crate::models::user_model::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: unsize,
    pub iat: unsize,
    pub user: User,
}
