use actix_web::{http::header::HeaderValue, web};
use jsonwebtoken::{DecodingKey, TokenData, Validation};

use crate::{
    models::{
        user::User,
        user_token::{UserToken, KEY},
    },
    utils::database_connection::Pool,
};

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub fn verify_token(
    token_data: &TokenData<UserToken>,
    pool: &web::Data<Pool>,
) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, pool.clone()) {
        Ok(token_data.claims.user.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}

pub fn is_auth_header_valid(authen_header: &HeaderValue) -> bool {
    if let Ok(authen_str) = authen_header.to_str() {
        return authen_str.starts_with("bearer") || authen_str.starts_with("Bearer");
    }

    return false;
}
