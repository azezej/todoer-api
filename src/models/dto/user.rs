use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
#[schema(example = json!({"id": 1, "first_name": "exampleName", "modified_at": ""}))]
pub struct UpdateUserFirstName {
    pub id: i32,
    pub first_name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct UpdateUserLastName {
    pub id: i32,
    pub last_name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct UpdateUserEmail {
    pub id: i32,
    pub email: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
#[schema(example = json!({"id": 1, "first_name": "exampleName", "modified_at": ""}))]
pub struct UpdateUserPassword {
    pub id: i32,
    pub password: String,
    pub modified_at: NaiveDateTime,
}
