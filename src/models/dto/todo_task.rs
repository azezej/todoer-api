use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskSummaryDTO {
    pub task_id: i32,
    pub summary: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskDescriptionDTO {
    pub task_id: i32,
    pub description: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskParentTaskDTO {
    pub task_id: i32,
    pub parent_task_id: i32,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskDueDateDTO {
    pub task_id: i32,
    pub due_date: NaiveDate,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskTodoListDTO {
    pub task_id: i32,
    pub todolist_id: i32,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskDoneDTO {
    pub task_id: i32,
    pub done: bool,
    pub modified_at: NaiveDateTime,
}
