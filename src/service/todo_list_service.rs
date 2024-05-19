use crate::constants;
use crate::error::ServiceError;
use crate::middleware::verify_auth;
use crate::models::todo_list::TodoListDTO;
use crate::{
    models::todo_list::TodoList, utils::database_connection::Pool,
    models::dto::todo_list::*,
};
use actix_web::http::header::HeaderValue;
use actix_web::web::{self};

pub fn get_all_lists(authen_header: &HeaderValue, pool: web::Data<Pool>) -> Result<Vec<TodoList>, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
        if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
            let uid = user.id;
            match TodoList::get_all_lists(uid, pool) {
                Ok(lists) => {
                    return Ok(lists);
                },
                Err(_) => {
                    return Err(ServiceError::NotFound { error_message: constants::MESSAGE_LISTS_NOT_FOUND_FOR_USER.to_string() });
                }
            }
        } else {
            return Err(ServiceError::BadRequest { error_message: constants::MESSAGE_TOKEN_MISSING.to_string() })
        }
    }
    Err(ServiceError::BadRequest { error_message: constants::MESSAGE_TOKEN_MISSING.to_string() })
}

pub fn db_get_list_by_id(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    list_id: i32,
) -> Result<TodoList, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
        if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
            let uid = user.id;
            match TodoList::db_get_list_by_id(list_id, uid, pool.clone()) {
                Ok(list) => {
                    return Ok(list);
                }, 
                Err(_) => {
                    return Err(ServiceError::NotFound { error_message: constants::MESSAGE_CAN_NOT_GET_LIST_BY_ID.to_string() });
                }
            }
        } else {
            return Err(ServiceError::BadRequest { error_message: constants::MESSAGE_TOKEN_MISSING.to_string() })
        }
    }
    Err(ServiceError::BadRequest { error_message: constants::MESSAGE_TOKEN_MISSING.to_string() })
}

pub fn create_list(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: TodoListDTO,
) -> Result<TodoList, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
    if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
        let uid = user.id;
        match TodoList::new(item, uid, pool) {
            Ok(list) => {
                return Ok(list);
            }, 
            Err(e) => {
                return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_CREATE_LIST.to_string() + " " + e.to_string().as_str() });
            }
        }
    } else {
        return Err(ServiceError::BadRequest {
            error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
        });
        }
    }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

pub fn delete_single_list(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    list_id: i32,
) -> Result<usize, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
        if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
            let uid = user.id;
            match TodoList::delete_single_list(list_id, uid, pool) {
                Ok(deletion) => {
                    return Ok(deletion);
                },
                Err(_) => {
                    return Err(ServiceError::NotFound { error_message: constants::MESSAGE_CAN_NOT_DELETE_LIST.to_string() });
                }
            }
        }
    }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

pub fn update_single_list_shared_with(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: web::Json<UpdateTodoListSharedWithDTO>,
) -> Result<TodoList, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
        if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
            let uid = user.id;
            match TodoList::update_single_list_shared_with(item.0, uid, pool) {
                Ok(update) => {
                    return Ok(update);
                },
                Err(_) => {
                    return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_UPDATE_LIST_SHARED_WITH.to_string() });
                }
            }
        }
    }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

pub fn update_single_list_parent_list_id(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: web::Json<UpdateTodoListParentListIdDTO>,
) -> Result<TodoList, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
        if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
            let uid = user.id;
            match TodoList::update_single_list_parent_list_id(item.0, uid, pool) {
                Ok(update) => {
                    return Ok(update);
                },
                Err(_) => {
                    return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_UPDATE_LIST_PARENT_LIST_ID.to_string() });
                }
            }
        }
    }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

pub fn update_single_list_name(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: web::Json<UpdateTodoListNameDTO>,
) -> Result<TodoList, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
        if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
            let uid = user.id;
            match TodoList::update_single_list_name(item.0, uid, pool) {
                Ok(update) => {
                    return Ok(update);
                },
                Err(_) => {
                    return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_UPDATE_LIST_NAME.to_string() });
                }
            }
        }
    }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

pub fn update_single_list_description(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: web::Json<UpdateTodoListDescriptionDTO>,
) -> Result<TodoList, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
        if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
            let uid = user.id;
            match TodoList::update_single_list_description(item.0, uid, pool) {
                Ok(update) => {
                    return Ok(update);
                },
                Err(_) => {
                    return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_UPDATE_LIST_DESCRIPTION.to_string() });
                }
            }
        }
    }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

