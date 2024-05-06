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
