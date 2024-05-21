use crate::constants;
use crate::error::ServiceError;
use crate::models::response::*;
use crate::models::db::todo_list::*;
use crate::service::todo_list_service;
use crate::{models::dto::todo_list::*, utils::database_connection::Pool};
use actix_web::web::{self};
use actix_web::{delete, get, patch, post, HttpRequest, HttpResponse};

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Creates a list for an authenticated user", body = TodoListResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
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

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Gets authenticated user's all lists", body = AllTodoListsResponse, example = json!({ "message": "ok", "data": [ { "id": 1, "user_id": 1, "todolist_id": 1, "summary": "test", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T19:18:17.816852", "modified_at": "2024-05-19T19:18:17.816913" }, { "id": 6, "user_id": 1, "todolist_id": 1, "summary": "123", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T23:03:01.748291", "modified_at": "2024-05-19T23:03:01.748352" } ] })),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!({ "message": "Internal Server Error", "data": "Error while trying to update list name." })),
    ),
)]
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

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Gets authenticated user's list", body = TodoListResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[get("/{id}")]
pub async fn get_list_by_id(req: HttpRequest, list_id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match todo_list_service::db_get_list_by_id(authen_header, pool, list_id.clone()) {
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

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Deletes authenticated user's list", body = TodoListDeletedResponse, example = json!({"message": "ok", "data": 1})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
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

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Patches authenticated user's list summary", body = TodoListResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[patch("/update/sharedwith/{id}")]
pub async fn patch_list_shared_with(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoListSharedWithDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let values = UpdateTodoListSharedWithDB {
            id: req.match_info().get("id").unwrap().parse().unwrap(),
            shared_with: input.shared_with.clone(),
            modified_at: chrono::Local::now().naive_local(),
        };
        match todo_list_service::update_single_list_shared_with(authen_header, pool, values) {
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

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Patches authenticated user's list summary", body = TodoListResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[patch("/update/parentlist/{id}")]
pub async fn patch_list_parent_list_id(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoListParentListIdDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let values = UpdateTodoListParentListIdDB {
            id: req.match_info().get("id").unwrap().parse().unwrap(),
            parent_list_id: input.parent_list_id.clone(),
            modified_at: chrono::Local::now().naive_local(),
        };
        match todo_list_service::update_single_list_parent_list_id(authen_header, pool, values) {
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

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Patches authenticated user's list title", body = TodoListResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[patch("/update/title/{id}")]
pub async fn patch_list_title(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoListTitleDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let values = UpdateTodoListTitleDB {
            id: req.match_info().get("id").unwrap().parse().unwrap(),
            title: input.title.clone(),
            modified_at: chrono::Local::now().naive_local(),
        };
        match todo_list_service::update_single_list_title(authen_header, pool, values) {
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

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Patches authenticated user's list description", body = TodoListResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[patch("/update/description/{id}")]
pub async fn patch_list_description(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoListDescriptionDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let values = UpdateTodoListDescriptionDB {
            id: req.match_info().get("id").unwrap().parse().unwrap(),
            description: input.description.clone(),
            modified_at: chrono::Local::now().naive_local(),
        };
        match todo_list_service::update_single_list_description(authen_header, pool, values) {
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
