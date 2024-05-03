use crate::routes::todo_list::*;
use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi};

#[derive(OpenApi)]
#[openapi(paths(
    add_list,
    get_lists,
    get_list_by_id,
    delete_list,
    patch_list_name,
    patch_list_description,
    patch_list_shared_with,
))]
pub struct TodoListApiDoc;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TodoList {
    pub id: i32,
    pub user_id: i32,
    pub shared_with: Option<String>,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = todolists)]
pub struct NewTodoList {
    pub user_id: i32,
    pub shared_with: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct InputTodoList {
    pub user_id: i32,
    pub shared_with: Option<String>,
    pub name: String,
    pub description: Option<String>,
}
