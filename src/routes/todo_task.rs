use crate::models::utils::tailored_response::*;
use crate::{
    models::dto::todo_task, models::todo_task::*, service::todo_task::todo_task_service::*,
    utils::database::connection::Pool,
};
use actix_web::web::{self};
use actix_web::{delete, get, patch, post, Error, HttpResponse, Responder};

#[post("/tasks/new")]
pub async fn add_task(
    db: web::Data<Pool>,
    item: web::Json<InputTodoTask>,
) -> Result<HttpResponse, Error> {
    match web::block(move || add_single_task(db, item)).await {
        Ok(task) => match serde_json::to_value(task.unwrap()) {
            Ok(response_body) => Ok(throw_response_created(response_body)),
            Err(e) => {
                eprintln!("Failed to create task: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(_) => Ok(throw_response_error()),
    }
}

#[get("/tasks")]
pub async fn get_tasks(db: web::Data<Pool>) -> HttpResponse {
    match web::block(move || get_all_tasks(db)).await {
        Ok(task) => match serde_json::to_value(task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to get tasks: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[get("/tasks/{id}")]
pub async fn get_task_by_id(db: web::Data<Pool>, task_id: web::Path<i32>) -> HttpResponse {
    match web::block(move || db_get_task_by_id(db, *task_id)).await {
        Ok(task) => match serde_json::to_value(task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to serialize task: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[delete("/tasks/{id}")]
pub async fn delete_task(
    db: web::Data<Pool>,
    task_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match web::block(move || delete_single_task(db, task_id.into_inner())).await {
        Ok(task) => match serde_json::to_value(task.unwrap()) {
            Ok(response_body) => Ok(throw_response_ok(response_body)),
            Err(e) => {
                eprintln!("Failed to delete task: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(_) => Ok(throw_response_error()),
    }
}

#[patch("/tasks/update/name/{id}")]
pub async fn patch_task_name(
    db: web::Data<Pool>,
    task_id: web::Json<todo_task::UpdateTodoTaskName>,
) -> impl Responder {
    match web::block(move || update_single_task_name(db, task_id)).await {
        Ok(updated_task) => match serde_json::to_value(updated_task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch task name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[patch("/tasks/update/description/{id}")]
pub async fn patch_task_description(
    db: web::Data<Pool>,
    task_id: web::Json<todo_task::UpdateTodoTaskDescription>,
) -> impl Responder {
    match web::block(move || update_single_task_description(db, task_id)).await {
        Ok(updated_task) => match serde_json::to_value(updated_task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch task description: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}
#[patch("/tasks/update/todolistid/{id}")]
pub async fn patch_task_todolist_id(
    db: web::Data<Pool>,
    task_id: web::Json<todo_task::UpdateTodoTaskTodoListID>,
) -> impl Responder {
    match web::block(move || update_single_task_todolist_id(db, task_id)).await {
        Ok(updated_task) => match serde_json::to_value(updated_task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch task todolist id: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}
#[patch("/tasks/update/duedate/{id}")]
pub async fn patch_task_due_date(
    db: web::Data<Pool>,
    task_id: web::Json<todo_task::UpdateTodoTaskDueDate>,
) -> impl Responder {
    match web::block(move || update_single_task_due_date(db, task_id)).await {
        Ok(updated_task) => match serde_json::to_value(updated_task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch task name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}
#[patch("/tasks/update/parenttaskid/{id}")]
pub async fn patch_task_parent_task_id(
    db: web::Data<Pool>,
    task_id: web::Json<todo_task::UpdateTodoTaskParentTaskID>,
) -> impl Responder {
    match web::block(move || update_single_task_parent_task_id(db, task_id)).await {
        Ok(updated_task) => match serde_json::to_value(updated_task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch task name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}
