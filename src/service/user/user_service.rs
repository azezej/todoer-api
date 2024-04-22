use crate::{
    diesel::{QueryDsl, RunQueryDsl},
    models::dto::user::*,
    models::user::{InputUser, NewUser, User},
    schema::users::dsl::*,
    utils::database::connection::Pool,
};
use actix_web::web::{self};
use diesel::{
    dsl::{delete, insert_into},
    ExpressionMethods,
};

pub fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = users.load::<User>(&mut conn)?;
    Ok(items)
}

pub fn db_get_user_by_id(
    pool: web::Data<Pool>,
    user_id: i32,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&mut conn)
}

pub fn add_single_user(
    pool: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let new_user = NewUser {
        first_name: item.first_name.clone(),
        last_name: item.last_name.clone(),
        email: item.email.clone(),
        created_at: chrono::Local::now().naive_local(),
        modified_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&mut conn)?;
    Ok(res)
}

pub fn update_user_email(
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

pub fn update_user_first_name(
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

pub fn update_user_last_name(
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

pub fn delete_single_user(
    db: web::Data<Pool>,
    user_id: i32,
) -> Result<usize, diesel::result::Error> {
    let mut conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&mut conn)?;
    Ok(count)
}
