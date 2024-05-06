use crate::{models::dto::todo_list, models::todo_list::*, utils::database_connection::Pool};
use actix_web::web::{self};
use actix_web::{delete, get, patch, post, Error, HttpResponse, Responder};

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Create list OK", body = String),
        (status = 500, description = "Create list FAILED", body = String)
    ),
)]
#[post("/create")]
pub async fn create_list(
    db: web::Data<Pool>,
    item: web::Json<TodoListDTO>,
) -> Result<HttpResponse, Error> {
}

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Get lists OK", body = String),
        (status = 500, description = "Get lists FAILED", body = String)
    )
)]
#[get("/")]
pub async fn get_lists(db: web::Data<Pool>) -> HttpResponse {
    match web::block(move || get_all_lists(db)).await {
        Ok(list) => match serde_json::to_value(list.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to get lists: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Get list by id OK", body = String),
        (status = 500, description = "Get list by id FAILED", body = String)
    )
)]
#[get("/{id}")]
pub async fn get_list_by_id(db: web::Data<Pool>, list_id: web::Path<i32>) -> HttpResponse {
    match web::block(move || db_get_list_by_id(db, *list_id)).await {
        Ok(list) => match serde_json::to_value(list.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to serialize list: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Delete list by id OK", body = String),
        (status = 500, description = "Delete list by id FAILED", body = String)
    ),

)]
#[delete("/{id}")]
pub async fn delete_list(
    db: web::Data<Pool>,
    list_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match web::block(move || delete_single_list(db, list_id.into_inner())).await {
        Ok(list) => match serde_json::to_value(list.unwrap()) {
            Ok(response_body) => Ok(throw_response_ok(response_body)),
            Err(e) => {
                eprintln!("Failed to delete list: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(_) => Ok(throw_response_error()),
    }
}

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Update lists name by id OK", body = String),
        (status = 500, description = "Update lists name by id FAILED", body = String)
    )
)]
#[patch("/update/name/{id}")]
pub async fn patch_list_name(
    db: web::Data<Pool>,
    list: web::Json<todo_list::UpdateTodoListName>,
) -> impl Responder {
    match web::block(move || update_single_list_name(db, list)).await {
        Ok(updated_list) => match serde_json::to_value(updated_list.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch list name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Update lists description by id OK", body = String),
        (status = 500, description = "Update lists description by id FAILED", body = String)
    )
)]
#[patch("/update/description/{id}")]
pub async fn patch_list_description(
    db: web::Data<Pool>,
    list: web::Json<todo_list::UpdateTodoListDescription>,
) -> impl Responder {
    match web::block(move || update_single_list_description(db, list)).await {
        Ok(updated_list) => match serde_json::to_value(updated_list.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch list description: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[utoipa::path(
    context_path = "/lists",
    responses(
        (status = 200, description = "Update lists shared with by id OK", body = String),
        (status = 500, description = "Update lists shared with by id FAILED", body = String)
    )
)]
#[patch("/update/sharedwith/{id}")]
pub async fn patch_list_shared_with(
    db: web::Data<Pool>,
    list: web::Json<todo_list::UpdateTodoListSharedWith>,
) -> impl Responder {
    match web::block(move || update_single_list_shared_with(db, list)).await {
        Ok(updated_list) => match serde_json::to_value(updated_list.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch list sharedwith: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}
