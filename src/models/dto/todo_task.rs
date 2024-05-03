use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoTaskName {
    pub task_id: i32,
    pub name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoTaskDescription {
    pub task_id: i32,
    pub description: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoTaskParentTaskID {
    pub task_id: i32,
    pub parent_task_id: i32,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoTaskDueDate {
    pub task_id: i32,
    pub due_date: NaiveDate,
    pub modified_at: NaiveDateTime,
}

/*
this below is maybe implementable if we plan to create a feature
to move tasks between workspaces or between users
*/

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoTaskTodoListID {
    pub task_id: i32,
    pub todolist_id: i32,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoTaskDone {
    pub task_id: i32,
    pub done: bool,
    pub modified_at: NaiveDateTime,
}
