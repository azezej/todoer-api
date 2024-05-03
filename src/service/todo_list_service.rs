use crate::diesel::{delete, insert_into, update, QueryDsl, RunQueryDsl};
use crate::models::{
    dto::todo_list,
    todo_list::{InputTodoList, NewTodoList},
};
use crate::{
    models::todo_list::TodoList, schema::todolists::dsl::*, utils::database_connection::Pool,
};
use actix_web::web::{self};
use diesel::prelude::*;

pub fn get_all_lists(pool: web::Data<Pool>) -> Result<Vec<TodoList>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = todolists.load::<TodoList>(&mut conn)?;
    Ok(items)
}

pub fn db_get_list_by_id(
    pool: web::Data<Pool>,
    list_id: i32,
) -> Result<TodoList, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    todolists.find(list_id).get_result::<TodoList>(&mut conn)
}

pub fn add_single_list(
    pool: web::Data<Pool>,
    item: web::Json<InputTodoList>,
) -> Result<TodoList, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let new_list = NewTodoList {
        user_id: item.user_id,
        name: item.name.clone(),
        description: item.description.clone(),
        shared_with: item.shared_with.clone(),
        created_at: chrono::Local::now().naive_local(),
        modified_at: chrono::Local::now().naive_local(),
    };

    let res = insert_into(todolists)
        .values(&new_list)
        .get_result(&mut conn)?;
    Ok(res)
}

pub fn delete_single_list(
    db: web::Data<Pool>,
    list_id: i32,
) -> Result<usize, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let deletion = delete(todolists.find(list_id)).execute(&mut conn)?;
    Ok(deletion)
}

pub fn update_single_list_name(
    db: web::Data<Pool>,
    item: web::Json<todo_list::UpdateTodoListName>,
) -> Result<TodoList, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let task = diesel::update(todolists)
        .set(name.eq(&item.name))
        .filter(id.eq(&item.id))
        .get_result(&mut conn)?;
    let _ = update(todolists)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.id));
    Ok(task)
}

pub fn update_single_list_description(
    db: web::Data<Pool>,
    item: web::Json<todo_list::UpdateTodoListDescription>,
) -> Result<TodoList, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let desc = item.description.clone().unwrap();
    let task = diesel::update(todolists)
        .set(description.eq(desc))
        .filter(id.eq(&item.id))
        .get_result(&mut conn)?;
    let _ = update(todolists)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.id));
    Ok(task)
}

pub fn update_single_list_shared_with(
    db: web::Data<Pool>,
    item: web::Json<todo_list::UpdateTodoListSharedWith>,
) -> Result<TodoList, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let task = diesel::update(todolists)
        .set(shared_with.eq(&item.shared_with))
        .filter(id.eq(&item.id))
        .get_result(&mut conn)?;
    let _ = update(todolists)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.id));
    Ok(task)
}
