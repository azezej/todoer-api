use actix_web::web;
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, Associations, Identifiable, Insertable, Queryable};

use crate::{
    models::user::User,
    schema::login_history::{self, dsl::*},
    utils::database_connection::Pool,
};

#[derive(Identifiable, Associations, Queryable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = login_history)]
pub struct LoginHistory {
    pub id: i32,
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = login_history)]
pub struct LoginHistoryInsertableDTO {
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime,
}

impl LoginHistory {
    pub fn create(un: &str, pool: &web::Data<Pool>) -> Option<LoginHistoryInsertableDTO> {
        if let Ok(user) = User::find_user_by_username(un, &pool) {
            let now = Utc::now();
            Some(LoginHistoryInsertableDTO {
                user_id: user.id,
                login_timestamp: now.naive_utc(),
            })
        } else {
            None
        }
    }

    pub fn save_login_history(
        insert_record: LoginHistoryInsertableDTO,
        pool: &web::Data<Pool>,
    ) -> QueryResult<usize> {
        let mut conn = pool.get().unwrap();
        diesel::insert_into(login_history)
            .values(&insert_record)
            .execute(&mut conn)
    }
}
