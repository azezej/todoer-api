use crate::constants;
use crate::error::ServiceError;
use crate::models::dto::todo_task::*;
use crate::models::response::ResponseBody;
use crate::service::todo_task_service;
use crate::{models::todo_task::TodoTaskDTO, models::db::todo_task::*, utils::database_connection::Pool};
use actix_web::{delete, get, patch, post, HttpResponse};
use actix_web::{web, HttpRequest};

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Creates a task for an authenticated user", body = TodoTaskResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
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

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Gets authenticated user's all tasks", body = AllTodoTasksResponse, example = json!({ "message": "ok", "data": [ { "id": 1, "user_id": 1, "todolist_id": 1, "summary": "test", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T19:18:17.816852", "modified_at": "2024-05-19T19:18:17.816913" }, { "id": 6, "user_id": 1, "todolist_id": 1, "summary": "123", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T23:03:01.748291", "modified_at": "2024-05-19T23:03:01.748352" } ] })),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!({ "message": "Internal Server Error", "data": "Error while trying to update task name." })),
    ),
)]
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

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Gets authenticated user's task", body = SampleTodoTaskResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
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

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Deletes authenticated user's task", body = SampleTodoTaskResponse, example = json!({"message": "ok", "data": 1})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
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

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patches authenticated user's task summary", body = TodoTaskResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[patch("/update/summary/{id}")]
pub async fn patch_task_summary(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskSummaryDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let values = UpdateTodoTaskSummaryDB {
            task_id: req.match_info().get("id").unwrap().parse().unwrap(),
            summary: input.summary.clone(),
            modified_at: chrono::Local::now().naive_local(),
        };
        match todo_task_service::update_single_task_summary(authen_header, pool, values) {
            Ok(response_body) => Ok(
                HttpResponse::Ok().json(ResponseBody::new(
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
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patches authenticated user's task description", body = TodoTaskResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[patch("/update/description/{id}")]
pub async fn patch_task_description(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskDescriptionDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let values = UpdateTodoTaskDescriptionDB {
            task_id: req.match_info().get("id").unwrap().parse().unwrap(),
            description: input.description.clone(),
            modified_at: chrono::Local::now().naive_local(),
        };
        match todo_task_service::update_single_task_description(authen_header, pool, values) {
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
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patches authenticated user's task todo list", body = TodoTaskResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[patch("/update/todolistid/{id}")]
pub async fn patch_task_todolist_id(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskTodoListDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let values = UpdateTodoTaskTodoListDB {
            task_id: req.match_info().get("id").unwrap().parse().unwrap(),
            todolist_id: input.todolist_id.clone(),
            modified_at: chrono::Local::now().naive_local(),
        };
        match todo_task_service::update_single_task_todolist_id(authen_header, pool, values) {
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
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patches authenticated user's task due date", body = TodoTaskResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[patch("/update/duedate/{id}")]
pub async fn patch_task_due_date(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskDueDateDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let values = UpdateTodoTaskDueDateDB {
            task_id: req.match_info().get("id").unwrap().parse().unwrap(),
            due_date: input.due_date.clone(),
            modified_at: chrono::Local::now().naive_local(),
        };
        match todo_task_service::update_single_task_due_date(authen_header, pool, values) {
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
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patches authenticated user's task parent task", body = TodoTaskResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[patch("/update/parenttaskid/{id}")]
pub async fn patch_task_parent_task_id(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskParentTaskDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let values = UpdateTodoTaskParentTaskDB {
            task_id: req.match_info().get("id").unwrap().parse().unwrap(),
            parent_task_id: input.parent_task_id.clone(),
            modified_at: chrono::Local::now().naive_local(),
        };
        match todo_task_service::update_single_task_parent_task_id(authen_header, pool, values) {
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
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patches authenticated user's task done", body = TodoTaskResponse, example = json!({"message": "ok", "data": { "id": 4, "user_id": 1, "todolist_id": 1, "summary": "hey", "description": "", "parent_task_id": 0, "due_date": null, "done": false, "created_at": "2024-05-19T22:58:41.378428", "modified_at": "2024-05-19T22:58:41.378492" }})),
        (status = 400, description = "Bad request", body = BadRequestResponse, example = json!("Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")),
        (status = 401, description = "Token missing", body = TokenMissingResponse, example = json!({"message": "Invalid token, please login again", "data": ""})),
        (status = 500, description = "Internal server error", body = InternalServerErrorResponse, example = json!("")),
    ),
)]
#[patch("/update/done/{id}")]
pub async fn patch_task_done(
    req: HttpRequest,
    pool: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskDoneDTO>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        let values = UpdateTodoTaskDoneDB {
            task_id: req.match_info().get("id").unwrap().parse().unwrap(),
            done: input.done.clone(),
            modified_at: chrono::Local::now().naive_local(),
        };
        match todo_task_service::update_single_task_done(authen_header, pool, values) {
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