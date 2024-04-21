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
pub struct NewTodoTask {
    pub user_id: i32,
    pub todolist_id: i32,
    pub name: String,
    pub description: Option<String>,
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
    pub description: Option<String>,
    pub parent_task_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskName {
    pub task_id: i32,
    pub name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskDescription {
    pub task_id: i32,
    pub description: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskParentTaskID {
    pub task_id: i32,
    pub parent_task_id: i32,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskDueDate {
    pub task_id: i32,
    pub due_date: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

/*
this below is maybe implementable if we plan to create a feature
to move tasks between workspaces or between users
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoTaskTodoListId {
    pub task_id: i32,
    pub todolist_id: i32,
    pub modified_at: NaiveDateTime,
}

