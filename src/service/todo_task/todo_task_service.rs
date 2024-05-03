use crate::schema::todotasks::dsl::*;
use crate::{models::dto::todo_task, models::todo_task::*, utils::database::connection::Pool};
use actix_web::web::{self};
use diesel::prelude::*;
use diesel::{delete, insert_into, update, QueryDsl, RunQueryDsl};

pub fn get_all_tasks(pool: web::Data<Pool>) -> Result<Vec<TodoTask>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = todotasks.load::<TodoTask>(&mut conn)?;
    Ok(items)
}

pub fn db_get_task_by_id(
    pool: web::Data<Pool>,
    task_id: i32,
) -> Result<TodoTask, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    todotasks.find(task_id).get_result::<TodoTask>(&mut conn)
}

pub fn add_single_task(
    pool: web::Data<Pool>,
    item: web::Json<InputTodoTask>,
) -> Result<TodoTask, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let new_task = NewTodoTask {
        user_id: item.user_id,
        todolist_id: item.todolist_id,
        name: item.name.clone(),
        description: item.description.clone(),
        parent_task_id: item.parent_task_id,
        done: false,
        due_date: item.due_date,
        created_at: chrono::Local::now().naive_local(),
        modified_at: chrono::Local::now().naive_local(),
    };

    let res = insert_into(todotasks)
        .values(&new_task)
        .get_result(&mut conn)?;
    Ok(res)
}

pub fn delete_single_task(
    db: web::Data<Pool>,
    task_id: i32,
) -> Result<usize, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let deletion = delete(todotasks.find(task_id)).execute(&mut conn)?;
    Ok(deletion)
}

pub fn update_single_task_name(
    db: web::Data<Pool>,
    item: web::Json<todo_task::UpdateTodoTaskName>,
) -> Result<TodoTask, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let task = diesel::update(todotasks)
        .set(name.eq(&item.name))
        .filter(id.eq(&item.task_id))
        .get_result(&mut conn)?;
    let _ = update(todotasks)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.task_id));
    Ok(task)
}

pub fn update_single_task_description(
    db: web::Data<Pool>,
    item: web::Json<todo_task::UpdateTodoTaskDescription>,
) -> Result<TodoTask, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let task = diesel::update(todotasks)
        .set(description.eq(&item.description))
        .filter(id.eq(&item.task_id))
        .get_result(&mut conn)?;
    let _ = update(todotasks)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.task_id));
    Ok(task)
}

pub fn update_single_task_parent_task_id(
    db: web::Data<Pool>,
    item: web::Json<todo_task::UpdateTodoTaskParentTaskID>,
) -> Result<TodoTask, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let task = diesel::update(todotasks)
        .set(parent_task_id.eq(&item.parent_task_id))
        .filter(id.eq(&item.task_id))
        .get_result(&mut conn)?;
    let _ = update(todotasks)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.task_id));
    Ok(task)
}

pub fn update_single_task_due_date(
    db: web::Data<Pool>,
    item: web::Json<todo_task::UpdateTodoTaskDueDate>,
) -> Result<TodoTask, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let task = diesel::update(todotasks)
        .set(due_date.eq(&item.due_date))
        .filter(id.eq(&item.task_id))
        .get_result(&mut conn)?;
    let _ = update(todotasks)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.task_id));
    Ok(task)
}

/*
this below is maybe implementable if we plan to create a feature
to move tasks between workspaces or between users
*/

pub fn update_single_task_todolist_id(
    db: web::Data<Pool>,
    item: web::Json<todo_task::UpdateTodoTaskTodoListID>,
) -> Result<TodoTask, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let task = diesel::update(todotasks)
        .set(todolist_id.eq(&item.todolist_id))
        .filter(id.eq(&item.task_id))
        .get_result(&mut conn)?;
    let _ = update(todotasks)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.task_id));
    Ok(task)
}

pub fn update_todo_task_done(
    db: web::Data<Pool>,
    item: web::Json<todo_task::UpdateTodoTaskDone>,
) -> Result<TodoTask, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let task = diesel::update(todotasks)
        .set(done.eq(&item.done))
        .filter(id.eq(&item.task_id))
        .get_result(&mut conn)?;
    let _ = update(todotasks)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.task_id));
    Ok(task)
}
