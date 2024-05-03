use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoListName {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoListDescription {
    pub id: i32,
    pub user_id: i32,
    pub description: Option<String>,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoListSharedWith {
    pub id: i32,
    pub user_id: i32,
    pub shared_with: String,
    pub modified_at: NaiveDateTime,
}
