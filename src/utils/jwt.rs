use crate::models::{jwt_model::Claims, user_model::Login};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

pub fn create_token_session(time_expiration_min: i64, user: Login) -> String {
    let header = Header::new(Algorithm::HS512);
    let encoding_key = EncodingKey::from_secret("secret".as_ref());

    let expiration = (Utc::now() + Duration::minutes(time_expiration_min)).timestamp() as u64;
    let iat = Utc::now().timestamp() as u64;

    let my_claims = Claims {
        exp: expiration,
        iat,
        user,
    };

    return encode(&header, &my_claims, &encoding_key).expect("error en el token");
}
