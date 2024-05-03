use crate::{
    constants,
    models::{dto::user::*, response::*, user::UserDTO, utils::tailored_response::*},
    service::user::user_service::{self, *},
    utils::database::connection::Pool,
};
use actix_web::{
    delete, get, http::StatusCode, patch, post, web, Error, HttpResponse, Responder, ResponseError,
};

#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 200, description = "Get user by id", body = String),
        (status = 500, description = "Get user by id failed", body = String)
    )
)]
#[get("/{id}")]
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match web::block(move || db_get_user_by_id(db, user_id.into_inner())).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => Ok(throw_response_ok(response_body)),
            Err(e) => {
                eprintln!("Failed to serialize user: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(e) => {
            eprintln!("Failed to retrieve user: {}", e);
            Ok(throw_response_error())
        }
    }
}

#[utoipa::path(post,
        context_path = "/users",
        responses(
            (status = 200, description = "Create user", body = String),
            (status = 500, description = "Create user failed", body = String)
        )
    )]
#[post("/register")]
pub async fn add_user(
    db: web::Data<Pool>,
    user_dto: web::Json<UserDTO>,
) -> Result<HttpResponse, Error> {
    match user_service::signup(user_dto.0, db) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_LOGIN_SUCCESS,
            token_res,
        ))),
        Err(_err) => Ok(HttpResponse::new(StatusCode::OK)),
    }
}
#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 200, description = "Delete user by id", body = String),
        (status = 500, description = "Delete user by id failed", body = String)
    )
)]
#[delete("/{id}")]
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match web::block(move || delete_single_user(db, user_id.into_inner())).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => Ok(throw_response_ok(response_body)),
            Err(e) => {
                eprintln!("Failed to delete user: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(_) => Ok(throw_response_error()),
    }
}

#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 200, description = "Patch user's first name by id", body = String),
        (status = 500, description = "Patch user's first name by id failed", body = String)
    )
)]
#[patch("/update/firstname/{id}")]
pub async fn patch_user_first_name(
    db: web::Data<Pool>,
    item: web::Json<UpdateUserFirstName>,
) -> impl Responder {
    match web::block(move || update_user_first_name(db, item)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch user's first name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 200, description = "Patch user's last name by id", body = String),
        (status = 500, description = "Patch user's last name by id failed", body = String)
    )
)]
#[patch("/update/lastname/{id}")]
pub async fn patch_user_last_name(
    db: web::Data<Pool>,
    item: web::Json<UpdateUserLastName>,
) -> impl Responder {
    match web::block(move || update_user_last_name(db, item)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch user's first name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 200, description = "Patch user's email by id", body = String),
        (status = 500, description = "Patch user's email by id failed", body = String)
    )
)]
#[patch("/update/email/{id}")]
pub async fn patch_user_email(
    db: web::Data<Pool>,
    item: web::Json<UpdateUserEmail>,
) -> impl Responder {
    match web::block(move || update_user_email(db, item)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch user's first name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}
