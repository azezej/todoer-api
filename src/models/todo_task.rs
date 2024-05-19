use crate::models::dto::todo_task::*;
use crate::schema::todotasks::{self, dsl::*};
use crate::utils::database_connection::Pool;
use actix_web::web;
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::result::Error;
use diesel::{prelude::*, Queryable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, JsonSchema, ApiComponent)]
pub struct TodoTask {
    pub id: i32,
    pub user_id: i32,
    pub todolist_id: i32,
    pub summary: String,
    pub description: Option<String>,
    pub parent_task_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
    pub done: bool,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug, JsonSchema, ApiComponent)]
#[diesel(table_name = todotasks)]
pub struct TodoTaskDTO {
    pub todolist_id: i32,
    pub summary: String,
    pub description: Option<String>,
    pub parent_task_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Insertable, Deserialize)]
#[diesel(table_name = todotasks)]
pub struct InputTodoTask {
    pub user_id: i32,
    pub todolist_id: i32,
    pub summary: String,
    pub description: Option<String>,
    pub parent_task_id: Option<i32>,
    pub done: bool,
    pub due_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

impl TodoTask {
    pub fn new(
        new_task: TodoTaskDTO,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoTask, Error> {
        let mut conn = pool.get().unwrap();

        let task = InputTodoTask {
            user_id: uid,
            todolist_id: new_task.todolist_id.clone(),
            summary: new_task.summary.clone(),
            description: new_task.description.clone(),
            parent_task_id: new_task.parent_task_id.clone(),
            done: false,
            due_date: new_task.due_date.clone(),
            created_at: chrono::Local::now().naive_local(),
            modified_at: chrono::Local::now().naive_local(),
        };
        let effect = diesel::insert_into(todotasks)
            .values(&task)
            .returning(todotasks::all_columns)
            .get_result::<TodoTask>(&mut conn);
        match effect {
           Ok(task) => {
               return Ok(task);
           },
           Err(e) => {
               return Err(e);
           } 
        }
    }

    pub fn get_all_tasks(
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<Vec<TodoTask>, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        let items = todotasks
            .filter(user_id.eq(uid))
            .load::<TodoTask>(&mut conn)?;
        Ok(items) 
    }

    pub fn db_get_task_by_id(
        task_id: i32,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        todotasks
            .filter(user_id.eq(uid))
            .filter(id.eq(task_id))
            .get_result::<TodoTask>(&mut conn)
    }

    pub fn delete_single_task(
        task_id: i32,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        let deletion = diesel::delete(todotasks.filter(user_id.eq(uid)).filter(id.eq(task_id)))
            .execute(&mut conn)?;
        Ok(deletion)
    }

    pub fn update_single_task_summary(
        item: UpdateTodoTaskSummaryDTO,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        let task = diesel::update(todotasks)
            .set(summary.eq(item.summary))
            .filter(user_id.eq(uid))
            .filter(id.eq(item.task_id))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(user_id.eq(uid))
            .filter(id.eq(item.task_id));
        Ok(task)
    }

    pub fn update_single_task_description(
        item: UpdateTodoTaskDescriptionDTO,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        let task = diesel::update(todotasks)
            .set(description.eq(item.description))
            .filter(id.eq(item.task_id))
            .filter(user_id.eq(uid))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(id.eq(item.task_id));
        Ok(task)
    }

    pub fn update_single_task_parent_task_id(
        item: UpdateTodoTaskParentTaskDTO,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        let task = diesel::update(todotasks)
            .set(parent_task_id.eq(item.parent_task_id))
            .filter(id.eq(item.task_id))
            .filter(user_id.eq(uid))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(id.eq(item.task_id))
            .filter(user_id.eq(uid));
        Ok(task)
    }

    pub fn update_single_task_due_date(
        item: UpdateTodoTaskDueDateDTO,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        let task = diesel::update(todotasks)
            .set(due_date.eq(item.due_date))
            .filter(user_id.eq(uid))
            .filter(id.eq(item.task_id))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(user_id.eq(uid))
            .filter(id.eq(item.task_id));
        Ok(task)
    }

    pub fn update_single_task_todolist_id(
        item: UpdateTodoTaskTodoListDTO,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        let task = diesel::update(todotasks)
            .set(todolist_id.eq(item.todolist_id))
            .filter(user_id.eq(uid))
            .filter(id.eq(item.task_id))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(id.eq(item.task_id));
        Ok(task)
    }

    pub fn update_single_task_done (
        item: UpdateTodoTaskDoneDTO,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        let task = diesel::update(todotasks)
            .set(done.eq(item.done))
            .filter(user_id.eq(uid))
            .filter(id.eq(item.task_id))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(user_id.eq(uid))
            .filter(id.eq(item.task_id));
        Ok(task)
    }
}
