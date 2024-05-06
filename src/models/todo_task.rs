use crate::models::dto::todo_task::*;
use crate::schema::todotasks::{self, dsl::*};
use crate::{routes::todo_task::*, utils::database_connection::Pool};
use actix_web::web;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi};
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
pub struct TodoTaskDTO {
    pub user_id: i32,
    pub todolist_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_task_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Insertable, Deserialize, IntoParams)]
#[diesel(table_name = todotasks)]
pub struct InputTodoTask {
    pub user_id: i32,
    pub todolist_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_task_id: Option<i32>,
    pub done: bool,
    pub due_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

impl TodoTask {
    pub fn new(new_task: TodoTaskDTO, pool: web::Data<Pool>) -> QueryResult<usize> {
        let mut conn = pool.get().unwrap();
        diesel::insert_into(todotasks)
            .values(new_task)
            .execute(&mut conn)
    }

    pub fn get_all_tasks(pool: web::Data<Pool>) -> Result<Vec<TodoTask>, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        let items = todotasks.load::<TodoTask>(&mut conn)?;
        Ok(items)
    }

    pub fn db_get_task_by_id(
        task_id: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        todotasks.find(task_id).get_result::<TodoTask>(&mut conn)
    }

    pub fn delete_single_task(
        db: web::Data<Pool>,
        task_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = db.get().unwrap();
        let deletion = diesel::delete(todotasks.find(task_id)).execute(&mut conn)?;
        Ok(deletion)
    }

    pub fn update_single_task_name(
        db: web::Data<Pool>,
        item: UpdateTodoTaskNameDTO,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = db.get().unwrap();
        let task = diesel::update(todotasks)
            .set(name.eq(&item.name))
            .filter(id.eq(&item.task_id))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(id.eq(&item.task_id));
        Ok(task)
    }

    pub fn update_single_task_description(
        db: web::Data<Pool>,
        item: UpdateTodoTaskDescriptionDTO,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = db.get().unwrap();
        let task = diesel::update(todotasks)
            .set(description.eq(&item.description))
            .filter(id.eq(&item.task_id))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(id.eq(&item.task_id));
        Ok(task)
    }

    pub fn update_single_task_parent_task_id(
        db: web::Data<Pool>,
        item: UpdateTodoTaskParentTaskDTO,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = db.get().unwrap();
        let task = diesel::update(todotasks)
            .set(parent_task_id.eq(&item.parent_task_id))
            .filter(id.eq(&item.task_id))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(id.eq(&item.task_id));
        Ok(task)
    }

    pub fn update_single_task_due_date(
        db: web::Data<Pool>,
        item: UpdateTodoTaskDueDateDTO,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = db.get().unwrap();
        let task = diesel::update(todotasks)
            .set(due_date.eq(&item.due_date))
            .filter(id.eq(&item.task_id))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(id.eq(&item.task_id));
        Ok(task)
    }

    pub fn update_single_task_todolist_id(
        db: web::Data<Pool>,
        item: UpdateTodoTaskTodoListDTO,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = db.get().unwrap();
        let task = diesel::update(todotasks)
            .set(todolist_id.eq(&item.todolist_id))
            .filter(id.eq(&item.task_id))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(id.eq(&item.task_id));
        Ok(task)
    }

    pub fn update_todo_task_done(
        db: web::Data<Pool>,
        item: UpdateTodoTaskDoneDTO,
    ) -> Result<TodoTask, diesel::result::Error> {
        let mut conn = db.get().unwrap();
        let task = diesel::update(todotasks)
            .set(done.eq(&item.done))
            .filter(id.eq(&item.task_id))
            .get_result(&mut conn)?;
        let _ = diesel::update(todotasks)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(id.eq(&item.task_id));
        Ok(task)
    }
}
