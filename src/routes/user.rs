use crate::{
    constants,
    error::ServiceError,
    models::{dto::user::*, response::*},
    service::user_service,
    utils::database_connection::Pool,
};
use actix_web::{
    delete, get, http::StatusCode, patch, post, web, Error, HttpRequest, HttpResponse, Responder,
};

#[utoipa::path(post,
        context_path = "/users",
        responses(
            (status = 200, description = "Create user", body = String),
            (status = 500, description = "Create user failed", body = String)
        )
    )]
#[post("/auth/register")]
pub async fn register_user(
    user_dto: web::Json<UserDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match user_service::signup(user_dto.0, &pool) {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_SIGNUP_SUCCESS,
            message,
        ))),
        Err(err) => Err(err),
    }
}

#[post("/auth/login")]
pub async fn login(
    login_dto: web::Json<LoginDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match user_service::login(login_dto.0, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_LOGIN_SUCCESS,
            token_res,
        ))),
        Err(err) => Err(err),
    }
}

#[post("/auth/logout")]
pub async fn logout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        user_service::logout(authen_header, &pool);
        Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_LOGOUT_SUCCESS,
            constants::EMPTY,
        )))
    } else {
        Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}

#[get("/auth/me")]
pub async fn me(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match user_service::me(authen_header, &pool) {
            Ok(login_info) => {
                Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, login_info)))
            }
            Err(err) => Err(err),
        }
    } else {
        Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}
