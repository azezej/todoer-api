use crate::constants;
use crate::models::dto::todo_task::*;
use crate::models::response::ResponseBody;
use crate::{
    models::dto::todo_task, models::todo_task::*, service::todo_task_service::*,
    utils::database_connection::Pool,
};
use actix_web::web;
use actix_web::{delete, get, http::StatusCode, patch, post, Error, HttpResponse, Responder};

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Create task by id OK", body = String),
        (status = 500, description = "Create task by id FAILED", body = String)
    )
)]
#[post("/create")]
pub async fn create_task(
    db: web::Data<Pool>,
    item: web::Json<TodoTaskDTO>,
) -> Result<HttpResponse, Error> {
    match TodoTask::new(item.0, db) {
        Ok(response_body) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_LOGIN_SUCCESS,
            response_body,
        ))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
            constants::EMPTY,
        ))),
    }
}

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Get all tasks OK", body = String),
        (status = 500, description = "Get all tasks FAILED", body = String)
    )
)]
#[get("/")]
pub async fn get_tasks(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    match TodoTask::get_all_tasks(db) {
        Ok(tasks) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, tasks)))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
            constants::EMPTY,
        ))),
    }
}

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Get task by id OK", body = String),
        (status = 500, description = "Get task by id FAILED", body = String)
    )
)]
#[get("/{id}")]
pub async fn get_task_by_id(db: web::Data<Pool>, task_id: web::Path<i32>) -> HttpResponse {
    match TodoTask::db_get_task_by_id(*task_id, db) {
        Ok(task) => {
            HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, task))
        }
        Err(_) => HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
            constants::EMPTY,
        )),
    }
}

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Delete task by id OK", body = String),
        (status = 500, description = "Delete task by id FAILED", body = String)
    )
)]
#[delete("/{id}")]
pub async fn delete_task(
    db: web::Data<Pool>,
    task_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match TodoTask::delete_single_task(db, *task_id) {
        Ok(_) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
            constants::EMPTY,
        ))),
    }
}

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patch task name by id OK", body = String),
        (status = 500, description = "Patch task name by id FAILED", body = String)
    )
)]
#[patch("/update/name/{id}")]
pub async fn patch_task_name(
    db: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskNameDTO>,
) -> Result<HttpResponse, Error> {
    match TodoTask::update_single_task_name(db, input.0) {
        Ok(task) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, task))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
            constants::EMPTY,
        ))),
    }
}

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patch task description by id OK", body = String),
        (status = 500, description = "Patch task description by id FAILED", body = String)
    )
)]
#[patch("/update/description/{id}")]
pub async fn patch_task_description(
    db: web::Data<Pool>,
    input: web::Json<UpdateTodoTaskDescriptionDTO>,
) -> Result<HttpResponse, Error> {
    match TodoTask::update_single_task_description(db, input.0) {
        Ok(task) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, task))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
            constants::EMPTY,
        ))),
    }
}

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patch task todolist id by id OK", body = String),
        (status = 500, description = "Patch task todolist id by id FAILED", body = String)
    )
)]
#[patch("/update/todolistid/{id}")]
pub async fn patch_task_todolist_id(
    db: web::Data<Pool>,
    result: web::Json<UpdateTodoTaskTodoListDTO>,
) -> Result<HttpResponse, Error> {
    match TodoTask::update_single_task_todolist_id(db, result.0) {
        Ok(task) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, task))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
            constants::EMPTY,
        ))),
    }
}

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patch task duedate by id OK", body = String),
        (status = 500, description = "Patch task duedate by id FAILED", body = String)
    )
)]
#[patch("/update/duedate/{id}")]
pub async fn patch_task_due_date(
    db: web::Data<Pool>,
    result: web::Json<UpdateTodoTaskDueDateDTO>,
) -> Result<HttpResponse, Error> {
    match TodoTask::update_single_task_due_date(db, result.0) {
        Ok(task) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, task))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
            constants::EMPTY,
        ))),
    }
}

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patch task parent task id by id OK", body = String),
        (status = 500, description = "Patch task parent task id by id FAILED", body = String)
    )
)]
#[patch("/update/parenttaskid/{id}")]
pub async fn patch_task_parent_task_id(
    db: web::Data<Pool>,
    task_id: web::Json<UpdateTodoTaskParentTaskDTO>,
) -> Result<HttpResponse, Error> {
    match TodoTask::update_single_task_parent_task_id(db, task_id.0) {
        Ok(task) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, task))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
            constants::EMPTY,
        ))),
    }
}

#[utoipa::path(
    context_path = "/tasks",
    responses(
        (status = 200, description = "Patch task done by id OK", body = String),
        (status = 500, description = "Patch task done by id FAILED", body = String)
    )
)]
#[patch("/update/done/{id}")]
pub async fn patch_task_done(
    db: web::Data<Pool>,
    result: web::Json<UpdateTodoTaskDoneDTO>,
) -> Result<HttpResponse, Error> {
    match TodoTask::update_todo_task_done(db, result.0) {
        Ok(task) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, task))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
            constants::EMPTY,
        ))),
    }
}
