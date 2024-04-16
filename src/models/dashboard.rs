use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TodoList {
    pub id: i32,
    pub user_id: i32,
    pub shared_with: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = todolists)]
pub struct NewTodoList<'a> {
    pub user_id: &'a i32,
    pub shared_with: &'a str,
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTodoList {
    pub user_id: i32,
    pub shared_with: String,
    pub name: String,
    pub description: String,
}
