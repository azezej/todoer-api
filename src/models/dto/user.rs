use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
