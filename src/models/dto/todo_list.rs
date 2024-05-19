use crate::schema::todolists::{self};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema, IntoParams};

#[derive(Serialize, Deserialize, Insertable, Debug, ToSchema, IntoParams)]
#[diesel(table_name = todolists)]
pub struct TodoListDTO {
    pub shared_with: Option<String>,
    pub parent_list_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct UpdateTodoListNameDTO {
    pub id: i32,
    pub name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct UpdateTodoListDescriptionDTO {
    pub id: i32,
    pub description: Option<String>,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct UpdateTodoListSharedWithDTO {
    pub id: i32,
    pub shared_with: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct UpdateTodoListParentListIdDTO {
    pub id: i32,
    pub parent_list_id: i32,
    pub modified_at: NaiveDateTime,
}