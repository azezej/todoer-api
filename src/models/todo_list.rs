use crate::schema::todolists::{self, dsl::*};
use crate::utils::database_connection::Pool;
use crate::models::db::todo_list::*;
use crate::models::dto::todo_list::*;
use actix_web::web;
use chrono::NaiveDateTime;
use diesel::{prelude::*, update, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Queryable, utoipa::ToSchema, utoipa::ToResponse)]
pub struct TodoList {
    pub id: i32,
    pub user_id: i32,
    pub shared_with: Option<String>,
    pub parent_list_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = todolists)]
pub struct InputTodoList {
    pub user_id: i32,
    pub shared_with: Option<String>,
    pub parent_list_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

impl TodoList {
    pub fn new(
        new_item: TodoListDTO,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoList, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        let new_list = InputTodoList {
            user_id: uid,
            title: new_item.title.clone(),
            description: new_item.description.clone(),
            shared_with: new_item.shared_with.clone(),
            parent_list_id: new_item.parent_list_id.clone(),
            created_at: chrono::Local::now().naive_local(),
            modified_at: chrono::Local::now().naive_local(),
        };

        let effect = diesel::insert_into(todolists)
            .values(&new_list)
            .returning(todolists::all_columns)
            .get_result(&mut conn);

        match effect {
            Ok(task) => {
                Ok(task)
            }
            Err(e) => {
                Err(e)
            }
        } 
    }

    pub fn get_all_lists(uid: i32, pool: web::Data<Pool>) -> Result<Vec<TodoList>, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        let items = todolists.filter(user_id.eq(uid)).load::<TodoList>(&mut conn)?;
        Ok(items)
    }

    pub fn db_get_list_by_id(
        list_id: i32,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoList, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        todolists
            .filter(user_id.eq(uid))
            .find(list_id)
            .get_result::<TodoList>(&mut conn)
    }

    pub fn delete_single_list(
        list_id: i32,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        let deletion = diesel::delete(todolists.filter(user_id.eq(uid)).find(list_id)).execute(&mut conn)?;
        Ok(deletion)
    }

    pub fn update_single_list_title(
        item: UpdateTodoListTitleDB,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoList, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        let task = diesel::update(todolists)
            .set(title.eq(&item.title))
            .filter(user_id.eq(uid))
            .filter(id.eq(&item.id))
            .get_result(&mut conn)?;
        let _ = update(todolists)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(user_id.eq(uid))
            .filter(id.eq(&item.id));
        Ok(task)
    }

    pub fn update_single_list_description(
        item: UpdateTodoListDescriptionDB,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoList, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        let desc = item.description.clone().unwrap();
        let task = diesel::update(todolists)
            .set(description.eq(desc))
            .filter(id.eq(&item.id))
            .filter(user_id.eq(uid))
            .get_result(&mut conn)?;
        let _ = update(todolists)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(id.eq(&item.id))
            .filter(user_id.eq(uid));
        Ok(task)
    }

    pub fn update_single_list_shared_with(
        item: UpdateTodoListSharedWithDB,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoList, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        let task = diesel::update(todolists)
            .set(shared_with.eq(&item.shared_with))
            .filter(id.eq(&item.id))
            .filter(user_id.eq(uid))
            .get_result(&mut conn)?;
        let _ = update(todolists)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(user_id.eq(uid))
            .filter(id.eq(&item.id));
        Ok(task)
    }

    pub fn update_single_list_parent_list_id(
        item: UpdateTodoListParentListIdDB,
        uid: i32,
        pool: web::Data<Pool>,
    ) -> Result<TodoList, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        let task = diesel::update(todolists)
            .set(parent_list_id.eq(&item.id))
            .filter(id.eq(&item.id))
            .filter(user_id.eq(uid))
            .get_result(&mut conn)?;
        let _ = update(todolists)
            .set(modified_at.eq(chrono::Local::now().naive_local()))
            .filter(user_id.eq(uid))
            .filter(id.eq(&item.id));
        Ok(task)
    }    
}
