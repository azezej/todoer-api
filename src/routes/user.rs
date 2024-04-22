use crate::{
    models::dto::user::*, models::user::InputUser, models::utils::tailored_response::*,
    service::user::user_service::*, utils::database::connection::Pool,
};
use actix_web::{
    delete, get, patch, post,
    web::{self},
    Error, HttpResponse, Responder,
};

#[patch("/users/update/firstname/{id}")]
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

#[patch("/users/update/lastname/{id}")]
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

#[patch("/users/update/email/{id}")]
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

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> impl Responder {
    match web::block(move || get_all_users(db)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to serialize user: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[get("/users/{id}")]
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

#[post("/users")]
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    match web::block(move || add_single_user(db, item)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => Ok(throw_response_created(response_body)),
            Err(e) => {
                eprintln!("Failed to create user: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(_) => Ok(throw_response_error()),
    }
}

#[delete("/users/{id}")]
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
