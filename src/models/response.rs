use serde::{Deserialize, Serialize};

use super::{todo_list::TodoList, todo_task::TodoTask};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ("responsebody created");
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}

#[derive(Serialize)]
pub struct Page<T> {
    pub message: String,
    pub data: Vec<T>,
    pub page_num: i64,
    pub page_size: i64,
    pub total_elements: i64,
}

impl<T> Page<T> {
    #[allow(dead_code)]
    pub fn new(
        message: &str,
        data: Vec<T>,
        page_num: i64,
        page_size: i64,
        total_elements: i64,
    ) -> Page<T> {
        Page {
            message: message.to_string(),
            data,
            page_num,
            page_size,
            total_elements,
        }
    }
}

#[derive(utoipa::ToResponse, utoipa::ToSchema)]
pub struct TodoTaskResponse {
    pub message: String,
    pub data: TodoTask,
}

#[derive(utoipa::ToResponse, utoipa::ToSchema)]
pub struct AllTodoTasksResponse {
    pub message: String,
    pub data: Vec<TodoTask>,
}

#[derive(utoipa::ToResponse, utoipa::ToSchema)]
pub struct TodoTaskDeletedResponse {
    pub message: String,
    pub data: i32, 
}

#[derive(utoipa::ToResponse, utoipa::ToSchema)]
pub struct TodoListResponse {
    pub message: String,
    pub data: TodoList,
}

#[derive(utoipa::ToResponse, utoipa::ToSchema)]
pub struct AllTodoListsResponse {
    pub message: String,
    pub data: Vec<TodoList>,
}

#[derive(utoipa::ToResponse, utoipa::ToSchema)]
pub struct TodoListDeletedResponse {
    pub message: String,
    pub data: i32, 
}

#[derive(utoipa::ToResponse, utoipa::ToSchema)]
pub struct TokenMissingResponse {
    pub data: String,
    pub message: String,
}

#[derive(utoipa::ToResponse, utoipa::ToSchema)]
pub struct InternalServerErrorResponse {
    pub message: String,
}

#[derive(utoipa::ToResponse, utoipa::ToSchema)]
pub struct BadRequestResponse {
    pub message: String,
}