use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUserDTO {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub login_session: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserFirstName {
    pub id: i32,
    pub first_name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserLastName {
    pub id: i32,
    pub last_name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserEmail {
    pub id: i32,
    pub email: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserPassword {
    pub id: i32,
    pub password: String,
    pub modified_at: NaiveDateTime,
}
