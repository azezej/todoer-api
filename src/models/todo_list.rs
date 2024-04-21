use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTodoList {
    pub user_id: i32,
    pub shared_with: Option<String>,
    pub name: String,
    pub description: Option<String>,
}

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