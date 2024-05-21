use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoListTitleDB {
    pub id: i32,
    pub title: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoListDescriptionDB {
    pub id: i32,
    pub description: Option<String>,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoListSharedWithDB {
    pub id: i32,
    pub shared_with: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoListParentListIdDB {
    pub id: i32,
    pub parent_list_id: i32,
    pub modified_at: NaiveDateTime,
}
