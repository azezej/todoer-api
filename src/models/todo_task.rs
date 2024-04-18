use crate::schema::*;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TodoTask {
    pub id: i32,
    pub user_id: i32,
    pub todolist_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_task_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = todotasks)]
pub struct NewTodoTask<'a> {
    pub user_id: i32,
    pub todolist_id: i32,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub parent_task_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTodoTask {
    pub user_id: i32,
    pub todolist_id: i32,
    pub name: String,
    pub description: String,
    pub parent_task_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
}
