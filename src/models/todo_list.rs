use crate::schema::todolists::{self, dsl::*};
use crate::{routes::todo_list::*, utils::database_connection::Pool};
use actix_web::web;
use chrono::NaiveDateTime;
use diesel::{prelude::*, update, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    add_list,
    get_lists,
    get_list_by_id,
    delete_list,
    patch_list_name,
    patch_list_description,
    patch_list_shared_with,
))]
pub struct TodoListApiDoc;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TodoList {
    pub id: i32,
    pub user_id: i32,
    pub shared_with: Option<String>,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = todolists)]
pub struct TodoListDTO {
    pub user_id: i32,
    pub shared_with: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListName {
    pub id: i32,
    pub name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListDescription {
    pub id: i32,
    pub description: Option<String>,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoListSharedWith {
    pub id: i32,
    pub shared_with: String,
    pub modified_at: NaiveDateTime,
}

impl TodoList {
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
        item: web::Json<TodoListDTO>,
    ) -> Result<TodoList, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        let new_list = TodoListDTO {
            user_id: item.user_id,
            name: item.name.clone(),
            description: item.description.clone(),
            shared_with: item.shared_with.clone(),
            created_at: chrono::Local::now().naive_local(),
            modified_at: chrono::Local::now().naive_local(),
        };

        let res = diesel::insert_into(todolists)
            .values(&new_list)
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn delete_single_list(
        db: web::Data<Pool>,
        list_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = db.get().unwrap();
        let deletion = diesel::delete(todolists.find(list_id)).execute(&mut conn)?;
        Ok(deletion)
    }

    pub fn update_single_list_name(
        db: web::Data<Pool>,
        item: web::Json<UpdateTodoListName>,
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
        item: web::Json<UpdateTodoListDescription>,
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
        item: web::Json<UpdateTodoListSharedWith>,
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
}
