use crate::{
    models::dto::user::*,
    models::login_history::*,
    models::user_token::*,
    schema::users::{self, dsl::*},
    utils::database_connection::Pool,
};
use actix_jwt_auth_middleware::FromRequest;
use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::{prelude::*, Identifiable};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, IntoParams)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub login_session: String,
}

#[derive(Serialize, Deserialize, Clone, FromRequest)]
struct UserClaims {
    id: i32,
    role: Role,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Role {
    Admin,
    RegularUser,
}

impl User {
    pub fn signup(new_user: UserDTO, pool: web::Data<Pool>) -> Result<HttpResponse, String> {
        let mut conn = pool.get().unwrap();
        if Self::find_user_by_username(&new_user.username, &pool).is_err() {
            let new_user = UserDTO {
                password: hash(&new_user.password, DEFAULT_COST).unwrap(),
                ..new_user
            };
            diesel::insert_into(users)
                .values(new_user)
                .execute(&mut conn);
            Ok(actix_web::HttpResponse::Ok().finish())
        } else {
            Err(format!(
                "User '{}' is already registered",
                &new_user.username
            ))
        }
    }

    pub fn login(login: LoginDTO, pool: web::Data<Pool>) -> Option<LoginInfoDTO> {
        let mut conn = pool.get().unwrap();
        if let Ok(user_to_verify) = users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(&mut conn)
        {
            if !user_to_verify.password.is_empty()
                && verify(&login.password, &user_to_verify.password).unwrap()
            {
                if let Some(login_history) = LoginHistory::create(&user_to_verify.username, &pool) {
                    if LoginHistory::save_login_history(login_history, &pool).is_err() {
                        return None;
                    }
                    let login_session_str = User::generate_login_session();
                    if User::update_login_session_to_db(
                        &user_to_verify.username,
                        &login_session_str,
                        pool,
                    ) {
                        return Some(LoginInfoDTO {
                            username: user_to_verify.username,
                            login_session: login_session_str,
                        });
                    }
                }
            } else {
                return Some(LoginInfoDTO {
                    username: user_to_verify.username,
                    login_session: String::new(),
                });
            }
        }

        None
    }

    pub fn logout(user_id: i32, pool: web::Data<Pool>) {
        let mut conn = pool.get().unwrap();
        if let Ok(user) = users.find(user_id).get_result::<User>(&mut conn) {
            Self::update_login_session_to_db(&user.username, "", pool);
        }
    }

    pub fn is_valid_login_session(user_token: &UserToken, pool: web::Data<Pool>) -> bool {
        let mut conn = pool.get().unwrap();
        users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(&mut conn)
            .is_ok()
    }

    pub fn find_login_info_by_token(
        user_token: &UserToken,
        pool: web::Data<Pool>,
    ) -> Result<LoginInfoDTO, String> {
        let mut conn = pool.get().unwrap();
        let user_result = users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(&mut conn);

        if let Ok(user) = user_result {
            return Ok(LoginInfoDTO {
                username: user.username,
                login_session: user.login_session,
            });
        }

        Err("User not found!".to_string())
    }

    pub fn find_user_by_username(un: &str, pool: &web::Data<Pool>) -> QueryResult<User> {
        let mut conn = pool.get().unwrap();
        users.filter(username.eq(un)).get_result::<User>(&mut conn)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn update_login_session_to_db(
        un: &str,
        login_session_str: &str,
        pool: web::Data<Pool>,
    ) -> bool {
        if let Ok(user) = User::find_user_by_username(un, &pool) {
            let mut conn = pool.get().unwrap();
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(&mut conn)
                .is_ok()
        } else {
            false
        }
    }
}
