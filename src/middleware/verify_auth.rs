use actix_web::{http::header::HeaderValue, web};

use crate::{constants, error::ServiceError, models::user::User, utils::{database_connection::Pool, token_utils}};

pub fn verify_user(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
) -> Result<User, ServiceError> {
    match check_token(authen_header, pool.clone()) { 
        Ok(username) => match User::find_user_by_username(&username, &pool) {
            Ok(user) => Ok(user),
            Err(_) => Err(ServiceError::Unauthorized {
                error_message: constants::MESSAGE_USER_NOT_FOUND.to_string(),
            })
        },
        Err(err) => Err(err),
    }
}

pub fn check_token(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
) -> Result<String, ServiceError> {
    if let Ok(authen_str) = authen_header.to_str() {
        if token_utils::is_auth_header_valid(authen_header) {
            let token = authen_str[6..authen_header.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(username) = token_utils::verify_token(&token_data, &pool) {
                    return Ok(username);
                } else {
                    return Err(ServiceError::Unauthorized {
                        error_message: constants::MESSAGE_USER_NOT_FOUND.to_string(),
                    });
                }
            } else {
                return Err(ServiceError::Unauthorized {
                    error_message: constants::MESSAGE_INVALID_TOKEN.to_string(),
                });
            }
        } else {
            return Err(ServiceError::Unauthorized {
                error_message: constants::MESSAGE_INVALID_TOKEN.to_string(),
            });
        }
    } else {
        return Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        });
    }
}