use crate::schema::*;
use chrono::NaiveDate;
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct InputTodoListTask {
    pub id: i32,
    pub user_id: i32,
    pub dashboard_id: i32,
    pub name: String,
    pub description: String,
    pub due_date: NaiveDate,
    pub parent_task_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = todotasks)]
pub struct NewTodoTask<'a> {
    pub user_id: &'a i32,
    pub dashboard_id: &'a i32,
    pub name: &'a str,
    pub description: &'a str,
    pub parent_task_id: &'a i32,
    pub due_date: NaiveDate,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTodoTask {
    pub user_id: i32,
    pub dashboard_id: i32,
    pub name: String,
    pub description: String,
    pub parent_task_id: i32,
    pub due_date: NaiveDate,
}
