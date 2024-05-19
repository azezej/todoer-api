use crate::constants;
use crate::error::ServiceError;
use crate::models::dto::todo_task::*;
use crate::models::response::ResponseBody;
use crate::service::todo_task_service;
use crate::{models::todo_task::*, utils::database_connection::Pool};
use actix_web::{delete, get, patch, post, HttpResponse};
use actix_web::{web, HttpRequest};

#[post("/create")]
pub async fn create_task(
    req: HttpRequest,
    item: web::Json<TodoTaskDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_task_service::create_task(authen_header, pool, item.0) {
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
pub async fn get_tasks(
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_task_service::get_all_tasks(authen_header, pool) {
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
pub async fn get_task_by_id(req: HttpRequest, task_id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_task_service::db_get_task_by_id(authen_header, pool, task_id.clone()) {
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
pub async fn delete_task(
    req: HttpRequest,
    pool: web::Data<Pool>,
    task_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError>{
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_task_service::delete_single_task(authen_header, pool, task_id.clone()) {
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
pub async fn patch_task_name(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskSummaryDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_task_service::update_single_task_name(authen_header, pool, input) {
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
pub async fn patch_task_description(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskDescriptionDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_task_service::update_single_task_description(authen_header, pool, input) {
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

#[patch("/update/todolistid/{id}")]
pub async fn patch_task_todolist_id(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskTodoListDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_task_service::update_single_task_todolist_id(authen_header, pool, input) {
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

#[patch("/update/duedate/{id}")]
pub async fn patch_task_due_date(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskDueDateDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_task_service::update_single_task_due_date(authen_header, pool, input) {
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

#[patch("/update/parenttaskid/{id}")]
pub async fn patch_task_parent_task_id(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskParentTaskDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_task_service::update_single_task_parent_task_id(authen_header, pool, input) {
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

#[patch("/update/done/{id}")]
pub async fn patch_task_done(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskDoneDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_task_service::update_single_task_done(authen_header, pool, input) {
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