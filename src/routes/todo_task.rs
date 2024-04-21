use crate::models::tailored_response::*;
use crate::schema::todotasks::dsl::*;
use crate::{models::todo_task::*, utils::database::connection::Pool};
use actix_web::web::{self};
use actix_web::{delete, get, patch, post, Error, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::{delete, insert_into, update, QueryDsl, RunQueryDsl};

fn get_all_tasks(pool: web::Data<Pool>) -> Result<Vec<TodoTask>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = todotasks.load::<TodoTask>(&mut conn)?;
    Ok(items)
}

fn db_get_task_by_id(
    pool: web::Data<Pool>,
    task_id: i32,
) -> Result<TodoTask, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    todotasks.find(task_id).get_result::<TodoTask>(&mut conn)
}

fn add_single_task(
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
        due_date: item.due_date,
        created_at: chrono::Local::now().naive_local(),
        modified_at: chrono::Local::now().naive_local(),
    };

    let res = insert_into(todotasks)
        .values(&new_task)
        .get_result(&mut conn)?;
    Ok(res)
}

fn delete_single_task(db: web::Data<Pool>, task_id: i32) -> Result<usize, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let deletion = delete(todotasks.find(task_id)).execute(&mut conn)?;
    Ok(deletion)
}

fn update_single_task_name(
    db: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskName>,
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

fn update_single_task_description(
    db: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskDescription>,
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

fn update_single_task_parent_task_id(
    db: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskParentTaskID>,
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

fn update_single_task_due_date(
    db: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskDueDate>,
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

fn update_single_task_todolist_id(
    db: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskTodoListID>,
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

#[post("/tasks/new")]
pub async fn add_task(
    db: web::Data<Pool>,
    item: web::Json<InputTodoTask>,
) -> Result<HttpResponse, Error> {
    match web::block(move || add_single_task(db, item)).await {
        Ok(task) => match serde_json::to_value(task.unwrap()) {
            Ok(response_body) => Ok(throw_response_created(response_body)),
            Err(e) => {
                eprintln!("Failed to create task: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(_) => Ok(throw_response_error()),
    }
}

#[get("/tasks")]
pub async fn get_tasks(db: web::Data<Pool>) -> HttpResponse {
    match web::block(move || get_all_tasks(db)).await {
        Ok(task) => match serde_json::to_value(task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to get tasks: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[get("/tasks/{id}")]
pub async fn get_task_by_id(db: web::Data<Pool>, task_id: web::Path<i32>) -> HttpResponse {
    match web::block(move || db_get_task_by_id(db, *task_id)).await {
        Ok(task) => match serde_json::to_value(task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to serialize task: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[delete("/tasks/{id}")]
pub async fn delete_task(
    db: web::Data<Pool>,
    task_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match web::block(move || delete_single_task(db, task_id.into_inner())).await {
        Ok(task) => match serde_json::to_value(task.unwrap()) {
            Ok(response_body) => Ok(throw_response_ok(response_body)),
            Err(e) => {
                eprintln!("Failed to delete task: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(_) => Ok(throw_response_error()),
    }
}

#[patch("/tasks/update/name/{id}")]
pub async fn patch_task_name(
    db: web::Data<Pool>,
    task_id: web::Json<UpdateTodoTaskName>,
) -> impl Responder {
    match web::block(move || update_single_task_name(db, task_id)).await {
        Ok(updated_task) => match serde_json::to_value(updated_task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch task name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[patch("/tasks/update/description/{id}")]
pub async fn patch_task_description(
    db: web::Data<Pool>,
    task_id: web::Json<UpdateTodoTaskDescription>,
) -> impl Responder {
    match web::block(move || update_single_task_description(db, task_id)).await {
        Ok(updated_task) => match serde_json::to_value(updated_task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch task description: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}
#[patch("/tasks/update/todolistid/{id}")]
pub async fn patch_task_todolist_id(
    db: web::Data<Pool>,
    task_id: web::Json<UpdateTodoTaskTodoListID>,
) -> impl Responder {
    match web::block(move || update_single_task_todolist_id(db, task_id)).await {
        Ok(updated_task) => match serde_json::to_value(updated_task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch task todolist id: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}
#[patch("/tasks/update/duedate/{id}")]
pub async fn patch_task_due_date(
    db: web::Data<Pool>,
    task_id: web::Json<UpdateTodoTaskDueDate>,
) -> impl Responder {
    match web::block(move || update_single_task_due_date(db, task_id)).await {
        Ok(updated_task) => match serde_json::to_value(updated_task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch task name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}
#[patch("/tasks/update/parenttaskid/{id}")]
pub async fn patch_task_parent_task_id(
    db: web::Data<Pool>,
    task_id: web::Json<UpdateTodoTaskParentTaskID>,
) -> impl Responder {
    match web::block(move || update_single_task_parent_task_id(db, task_id)).await {
        Ok(updated_task) => match serde_json::to_value(updated_task.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch task name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}
