use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListName {
    pub user_id: i32,
    pub name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListDescription {
    pub user_id: i32,
    pub description: Option<String>,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListSharedWith {
    pub user_id: i32,
    pub shared_with: String,
    pub modified_at: NaiveDateTime,
}
