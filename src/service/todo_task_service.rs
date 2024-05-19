use crate::constants;
use crate::middleware::verify_auth;
use crate::error::ServiceError;
use crate::{models::dto::todo_task::*, models::todo_task::*, utils::database_connection::Pool};
use actix_web::http::header::HeaderValue;
use actix_web::web::{self};

pub fn get_all_tasks(authen_header: &HeaderValue, pool: web::Data<Pool>) -> Result<Vec<TodoTask>, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
                    if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
                        let uid = user.id;
                        match TodoTask::get_all_tasks(uid, pool) {
                            Ok(tasks) => {
                                return Ok(tasks);
                            },
                            Err(_) => {
                                return Err(ServiceError::NotFound { error_message: constants::MESSAGE_TASKS_NOT_FOUND_FOR_USER.to_string() });
                            }
                        }
                    } else {
                        return Err(ServiceError::BadRequest { error_message: constants::MESSAGE_TOKEN_MISSING.to_string() })
                    }
                }
    Err(ServiceError::BadRequest { error_message: constants::MESSAGE_TOKEN_MISSING.to_string() })
}

pub fn db_get_task_by_id(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    task_id: i32,
) -> Result<TodoTask, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
                    if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
                        let uid = user.id;
                        match TodoTask::db_get_task_by_id(task_id, uid, pool.clone()) {
                            Ok(task) => {
                                return Ok(task);
                            }, 
                            Err(e) => {
                                return Err(ServiceError::NotFound { error_message: constants::MESSAGE_CAN_NOT_GET_TASK_BY_ID.to_string() + " " + &e.to_string() });
                            }
                        }
                    } else {
                        return Err(ServiceError::BadRequest { error_message: constants::MESSAGE_TOKEN_MISSING.to_string() })
                    }
                }
                Err(ServiceError::BadRequest { error_message: constants::MESSAGE_TOKEN_MISSING.to_string() })
}

pub fn create_task(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: TodoTaskDTO,
) -> Result<TodoTask, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
        if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
            let uid = user.id;
            println!("uid: {}", uid);
            match TodoTask::new(item, uid, pool) {
                Ok(task) => {
                    return Ok(task);
                }, 
                Err(e) => {
                    return Err(ServiceError::InternalServerError { error_message: e.to_string() });
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

pub fn delete_single_task(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    task_id: i32,
) -> Result<usize, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
                    if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
                        let uid = user.id;
                        match TodoTask::delete_single_task(task_id, uid, pool) {
                            Ok(deletion) => {
                                return Ok(deletion);
                            },
                            Err(_) => {
                                return Err(ServiceError::NotFound { error_message: constants::MESSAGE_CAN_NOT_DELETE_TASK.to_string() });
                            }
                       }
                    }
                }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

pub fn update_single_task_name(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskNameDTO>,
) -> Result<TodoTask, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
                    if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
                        let uid = user.id;
                        match TodoTask::update_single_task_name(item.0, uid, pool) {
                            Ok(update) => {
                                return Ok(update);
                            },
                            Err(_) => {
                                return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_UPDATE_TASK_NAME.to_string() });
                            }
                       }
                    }
                }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

pub fn update_single_task_description(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskDescriptionDTO>,
) -> Result<TodoTask, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
                    if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
                        let uid = user.id;
                        match TodoTask::update_single_task_description(item.0, uid, pool) {
                            Ok(update) => {
                                return Ok(update);
                            },
                            Err(_) => {
                                return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_UPDATE_TASK_DESCRIPTION.to_string() });
                            }
                       }
                    }
                }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

pub fn update_single_task_due_date(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskDueDateDTO>,
) -> Result<TodoTask, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
                    if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
                        let uid = user.id;
                        match TodoTask::update_single_task_due_date(item.0, uid, pool) {
                            Ok(update) => {
                                return Ok(update);
                            },
                            Err(_) => {
                                return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_UPDATE_TASK_DUE_DATE.to_string() });
                            }
                       }
                    }
                }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

/*
this below is maybe implementable if we plan to create a feature
to move tasks between workspaces or between users
*/

pub fn update_single_task_todolist_id(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskTodoListDTO>,
) -> Result<TodoTask, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
                    if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
                        let uid = user.id;
                        match TodoTask::update_single_task_todolist_id(item.0, uid, pool) {
                            Ok(update) => {
                                return Ok(update);
                            },
                            Err(_) => {
                                return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_UPDATE_TASK_TODO_LIST.to_string() });
                            }
                       }
                    }
                }
            Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

pub fn update_single_task_done(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskDoneDTO>,
) -> Result<TodoTask, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
                    if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
                        let uid = user.id;
                        match TodoTask::update_single_task_done(item.0, uid, pool) {
                            Ok(update) => {
                                return Ok(update);
                            },
                            Err(_) => {
                                return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_UPDATE_TASK_DONE.to_string() });
                            }
                       }
                    }
                }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}

pub fn update_single_task_parent_task_id(
    authen_header: &HeaderValue,
    pool: web::Data<Pool>,
    item: web::Json<UpdateTodoTaskParentTaskDTO>,
) -> Result<TodoTask, ServiceError> {
   if let Ok(_) = verify_auth::check_token(authen_header, pool.clone()) {
                    if let Ok(user) = verify_auth::verify_user(&authen_header, pool.clone()) {
                        let uid = user.id;
                        match TodoTask::update_single_task_parent_task_id(item.0, uid, pool) {
                            Ok(update) => {
                                return Ok(update);
                            },
                            Err(_) => {
                                return Err(ServiceError::InternalServerError { error_message: constants::MESSAGE_CAN_NOT_UPDATE_TASK_DONE.to_string() });
                            }
                       }
                    }
                }
    Err(ServiceError::BadRequest {
        error_message: constants::MESSAGE_TOKEN_MISSING.to_string(),
    })
}