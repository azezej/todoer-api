use crate::{
    diesel::{QueryDsl, RunQueryDsl},
    models::user::{InputUser, NewUser, User},
    schema::users::dsl::*,
    utils::database::connection::Pool,
};
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use std::vec::Vec;

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = users.load::<User>(&mut conn)?;
    Ok(items)
}

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    match web::block(move || get_all_users(db)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(response_body)),
            Err(e) => {
                eprintln!("Failed to serialize user: {}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match web::block(move || db_get_user_by_id(db, user_id.into_inner())).await {
        Ok(user) => {
            // Serialize the user object
            match serde_json::to_value(user.unwrap()) {
                Ok(response_body) => Ok(HttpResponse::Ok().json(response_body)),
                Err(e) => {
                    eprintln!("Failed to serialize user: {}", e);
                    Ok(HttpResponse::InternalServerError().finish())
                }
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

// Handler for POST /users
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    match web::block(move || add_single_user(db, item)).await {
        Ok(user) => {
            // Serialize the user object
            match serde_json::to_value(user.unwrap()) {
                Ok(response_body) => Ok(HttpResponse::Created().json(response_body)),
                Err(e) => {
                    eprintln!("Failed to create user: {}", e);
                    Ok(HttpResponse::InternalServerError().finish())
                }
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

// Handler for DELETE /users/{id}
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match web::block(move || delete_single_user(db, user_id.into_inner())).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(response_body)),
            Err(e) => {
                eprintln!("Failed to create user: {}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&mut conn)
}

fn add_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&mut conn)?;
    Ok(res)
}

fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&mut conn)?;
    Ok(count)
}
