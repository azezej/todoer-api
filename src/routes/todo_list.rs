use crate::constants;
use crate::error::ServiceError;
use crate::models::response::ResponseBody;
use crate::service::todo_list_service;
use crate::{models::dto::todo_list::*, utils::database_connection::Pool};
use actix_web::web::{self};
use actix_web::{delete, get, patch, post, HttpRequest, HttpResponse};

#[post("/create")]
pub async fn create_list(
    req: HttpRequest,
    item: web::Json<TodoListDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_list_service::create_list(authen_header, pool, item.0) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_OK,
                response_body
            ))),
            Err(e) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
                constants::MESSAGE_INTERNAL_SERVER_ERROR,
                &e.to_string(),
            ))),
        }
    } else {
        Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}

#[get("/")]
pub async fn get_lists(
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_list_service::get_all_lists(authen_header, pool) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_OK,
                response_body
            ))),
            Err(e) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
                constants::MESSAGE_INTERNAL_SERVER_ERROR,
                &e.to_string(),
            ))),
        }
    } else {
        Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}

#[get("/{id}")]
pub async fn get_list_by_id(req: HttpRequest, task_id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_list_service::db_get_list_by_id(authen_header, pool, task_id.clone()) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_OK,
                response_body
            ))),
            Err(e) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
                constants::MESSAGE_INTERNAL_SERVER_ERROR,
                &e.to_string(),
            ))),
        }
    } else {
        Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}

#[delete("/{id}")]
pub async fn delete_list(
    req: HttpRequest,
    pool: web::Data<Pool>,
    list_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError>{
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_list_service::delete_single_list(authen_header, pool, list_id.clone()) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_OK,
                response_body
            ))),
            Err(e) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
                constants::MESSAGE_INTERNAL_SERVER_ERROR,
                &e.to_string(),
            ))),
        }
    } else {
        Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}

#[patch("/update/sharedwith/{id}")]
pub async fn patch_list_shared_with(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoListSharedWithDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_list_service::update_single_list_shared_with(authen_header, pool, input) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_OK,
                response_body
            ))),
            Err(e) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
                constants::MESSAGE_INTERNAL_SERVER_ERROR,
                &e.to_string(),
            ))),
        }
    } else {
        Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}

#[patch("/update/parentlist/{id}")]
pub async fn patch_list_parent_list_id(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoListParentListIdDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_list_service::update_single_list_parent_list_id(authen_header, pool, input) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_OK,
                response_body
            ))),
            Err(e) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
                constants::MESSAGE_INTERNAL_SERVER_ERROR,
                &e.to_string(),
            ))),
        }
    } else {
        Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}

#[patch("/update/name/{id}")]
pub async fn patch_list_name(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoListTitleDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_list_service::update_single_list_title(authen_header, pool, input) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_OK,
                response_body
            ))),
            Err(e) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
                constants::MESSAGE_INTERNAL_SERVER_ERROR,
                &e.to_string(),
            ))),
        }
    } else {
        Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}

#[patch("/update/description/{id}")]
pub async fn patch_list_description(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoListDescriptionDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_list_service::update_single_list_description(authen_header, pool, input) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_OK,
                response_body
            ))),
            Err(e) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
                constants::MESSAGE_INTERNAL_SERVER_ERROR,
                &e.to_string(),
            ))),
        }
    } else {
        Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        })
    }
}
