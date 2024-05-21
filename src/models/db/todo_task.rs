use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskSummaryDB {
    pub task_id: i32,
    pub summary: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskDescriptionDB {
    pub task_id: i32,
    pub description: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskParentTaskDB {
    pub task_id: i32,
    pub parent_task_id: i32,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskDueDateDB {
    pub task_id: i32,
    pub due_date: NaiveDate,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskTodoListDB {
    pub task_id: i32,
    pub todolist_id: i32,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskDoneDB {
    pub task_id: i32,
    pub done: bool,
    pub modified_at: NaiveDateTime,
}
