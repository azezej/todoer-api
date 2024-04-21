use crate::{
    diesel::{QueryDsl, RunQueryDsl},
    models::user::{
        InputUser, NewUser, UpdateUserEmail, UpdateUserFirstName, UpdateUserLastName, User,
    },
    schema::users::dsl::*,
    utils::database::connection::Pool,
    models::tailored_response::*
};
use actix_web::{
    delete, get, patch, post,
    web::{self},
    Error, HttpResponse, Responder,
};
use diesel::{
    dsl::{delete, insert_into},
    ExpressionMethods,
};
use std::vec::Vec;

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = users.load::<User>(&mut conn)?;
    Ok(items)
}

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&mut conn)
}

fn add_single_user(
    pool: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
        modified_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&mut conn)?;
    Ok(res)
}

fn update_user_email(
    pool: web::Data<Pool>,
    item: web::Json<UpdateUserEmail>,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let res_email = diesel::update(users)
        .set(email.eq(&item.email))
        .filter(id.eq(&item.id))
        .get_result(&mut conn)?;
    let _ = diesel::update(users)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.id));
    Ok(res_email)
}

fn update_user_first_name(
    pool: web::Data<Pool>,
    item: web::Json<UpdateUserFirstName>,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let res_first_name = diesel::update(users)
        .set(first_name.eq(&item.first_name))
        .filter(id.eq(&item.id))
        .get_result(&mut conn)?;
    let _ = diesel::update(users)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.id));
    Ok(res_first_name)
}

fn update_user_last_name(
    pool: web::Data<Pool>,
    item: web::Json<UpdateUserLastName>,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let res_email = diesel::update(users)
        .set(last_name.eq(&item.last_name))
        .filter(id.eq(&item.id))
        .get_result(&mut conn)?;
    let _ = diesel::update(users)
        .set(modified_at.eq(chrono::Local::now().naive_local()))
        .filter(id.eq(&item.id));
    Ok(res_email)
}

#[patch("/users/update/firstname/{id}")]
pub async fn patch_user_first_name(
    db: web::Data<Pool>,
    item: web::Json<UpdateUserFirstName>,
) -> impl Responder {
    match web::block(move || update_user_first_name(db, item)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch user's first name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[patch("/users/update/lastname/{id}")]
pub async fn patch_user_last_name(
    db: web::Data<Pool>,
    item: web::Json<UpdateUserLastName>,
) -> impl Responder {
    match web::block(move || update_user_last_name(db, item)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch user's first name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[patch("/users/update/email/{id}")]
pub async fn patch_user_email(
    db: web::Data<Pool>,
    item: web::Json<UpdateUserEmail>,
) -> impl Responder {
    match web::block(move || update_user_email(db, item)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to patch user's first name: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&mut conn)?;
    Ok(count)
}

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> impl Responder {
    match web::block(move || get_all_users(db)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => throw_response_ok(response_body),
            Err(e) => {
                eprintln!("Failed to serialize user: {}", e);
                throw_response_error()
            }
        },
        Err(_) => throw_response_error(),
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match web::block(move || db_get_user_by_id(db, user_id.into_inner())).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => Ok(throw_response_ok(response_body)),
            Err(e) => {
                eprintln!("Failed to serialize user: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(e) => {
            eprintln!("Failed to retrieve user: {}", e);
            Ok(throw_response_error())
        }
    }
}

#[post("/users")]
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    match web::block(move || add_single_user(db, item)).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => Ok(throw_response_created(response_body)),
            Err(e) => {
                eprintln!("Failed to create user: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(_) => Ok(throw_response_error()),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match web::block(move || delete_single_user(db, user_id.into_inner())).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => Ok(throw_response_ok(response_body)),
            Err(e) => {
                eprintln!("Failed to delete user: {}", e);
                Ok(throw_response_error())
            }
        },
        Err(_) => Ok(throw_response_error()),
    }
}
