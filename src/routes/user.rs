use crate::{
    diesel::{QueryDsl, RunQueryDsl},
    models::user::{InputUser, NewUser, UpdateUser, User},
    schema::users::dsl::*,
    utils::database::connection::Pool,
};
use actix_web::{delete, get, patch, post, web, Error, HttpResponse, Responder};
use diesel::{
    dsl::{delete, insert_into, Update},
    update, ExpressionMethods,
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

fn change_user_email(
    pool: web::Data<Pool>,
    item: web::Json<UpdateUser>,
) -> Result<UpdateUser, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let res_email = diesel::update(users)
        .set(email.eq(&item.email))
        .filter(id.eq(&item.id))
        .get_result::<UpdateUser>(&mut conn)?;
    Ok(res_email)
}

fn change_user_first_name(
    pool: web::Data<Pool>,
    item: web::Json<UpdateUser>,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let res_first_name = diesel::update(users)
        .set(first_name.eq(&item.first_name))
        .filter(id.eq(&item.id))
        .get_result(&mut conn)?;
    Ok(res_first_name)
}

fn change_user_last_name(
    pool: web::Data<Pool>,
    item: web::Json<UpdateUser>,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let res_email = diesel::update(users)
        .set(email.eq(&item.email))
        .filter(id.eq(&item.id))
        .get_result(&mut conn)?;
    Ok(res_email)
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
            Ok(response_body) => HttpResponse::Ok().json(response_body),
            Err(e) => {
                eprintln!("Failed to serialize user: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match web::block(move || db_get_user_by_id(db, user_id.into_inner())).await {
        Ok(user) => match serde_json::to_value(user.unwrap()) {
            Ok(response_body) => Ok(HttpResponse::Ok().json(response_body)),
            Err(e) => {
                eprintln!("Failed to serialize user: {}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        },
        Err(e) => {
            eprintln!("Failed to retrieve user: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
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
            Ok(response_body) => Ok(HttpResponse::Created().json(response_body)),
            Err(e) => {
                eprintln!("Failed to create user: {}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[delete("/users/{id}")]
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
