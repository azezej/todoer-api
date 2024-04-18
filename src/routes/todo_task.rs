use actix_web::{post, web, Error, HttpResponse};
use diesel::{insert_into, QueryDsl, RunQueryDsl};
use crate::schema::todotasks::dsl::*;
use crate::{models::todo_task::{InputTodoTask, TodoTask, NewTodoTask}, utils::database::connection::Pool};

fn get_all_tasks(pool: web::Data<Pool>) -> Result<Vec<TodoTask>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = todotasks.load::<TodoTask>(&mut conn)?;
    Ok(items)
}

fn get_task_by_id(pool: web::Data<Pool>, task_id: i32) -> Result<TodoTask, diesel::result::Error> {
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
        name: &item.name,
        description: Some(&item.description),
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

#[post("/tasks/new")]
pub async fn add_task(
    db: web::Data<Pool>,
    item: web::Json<InputTodoTask>,
) -> Result<HttpResponse, Error> {
    match web::block(move || add_single_task(db, item)).await {
        Ok(task) => match serde_json::to_value(task.unwrap()) {
            Ok(response_body) => Ok(HttpResponse::Created().json(response_body)),
            Err(e) => {
                eprintln!("Failed to create task: {}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

