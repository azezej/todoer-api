use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListNameDTO {
    pub id: i32,
    pub name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListDescriptionDTO {
    pub id: i32,
    pub description: Option<String>,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListSharedWithDTO {
    pub id: i32,
    pub shared_with: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListParentListIdDTO {
    pub id: i32,
    pub parent_list_id: i32,
    pub modified_at: NaiveDateTime,
}