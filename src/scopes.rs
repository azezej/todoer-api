use crate::routes;

pub fn user_scope() -> actix_web::Scope {
    actix_web::Scope::new("/users")
        .service(routes::user::login)
        .service(routes::user::register_user)
        .service(routes::user::me)
        .service(routes::user::logout)
}

pub fn todo_task_scope() -> actix_web::Scope {
    actix_web::Scope::new("/tasks")
        .service(routes::todo_task::get_tasks)
        .service(routes::todo_task::get_task_by_id)
        .service(routes::todo_task::create_task)
        .service(routes::todo_task::delete_task)
        .service(routes::todo_task::patch_task_name)
        .service(routes::todo_task::patch_task_description)
        .service(routes::todo_task::patch_task_due_date)
        .service(routes::todo_task::patch_task_todolist_id)
        .service(routes::todo_task::patch_task_parent_task_id)
}

pub fn todo_list_scope() -> actix_web::Scope {
    actix_web::Scope::new("/lists")
        .service(routes::todo_list::get_lists)
        .service(routes::todo_list::get_list_by_id)
        .service(routes::todo_list::create_list)
        .service(routes::todo_list::delete_list)
        .service(routes::todo_list::patch_list_name)
        .service(routes::todo_list::patch_list_description)
        .service(routes::todo_list::patch_list_shared_with)
        .service(routes::todo_list::patch_list_parent_list_id)
}