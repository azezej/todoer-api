use crate::routes::todo_task::*;
use crate::schema::*;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi};

#[derive(OpenApi)]
#[openapi(paths(
    add_task,
    get_tasks,
    get_task_by_id,
    delete_task,
    patch_task_name,
    patch_task_description,
    patch_task_todolist_id,
    patch_task_due_date,
    patch_task_parent_task_id,
))]
pub struct TodoTaskApiDoc;

#[derive(Debug, Serialize, Deserialize, Queryable, IntoParams)]
pub struct TodoTask {
    pub id: i32,
    pub user_id: i32,
    pub todolist_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_task_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
    pub done: bool,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug, IntoParams)]
#[diesel(table_name = todotasks)]
pub struct NewTodoTask {
    pub user_id: i32,
    pub todolist_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_task_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
    pub done: bool,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct InputTodoTask {
    pub user_id: i32,
    pub todolist_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_task_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
}
