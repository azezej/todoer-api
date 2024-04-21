extern crate diesel;

use actix_web::{web::Data, App, HttpServer};

mod models {
    pub mod todo_list;
    pub mod todo_task;
    pub mod tailored_response;
    pub mod user;
}
mod routes {
    pub mod todo_task;
    pub mod user;
}
mod utils {
    pub mod database {
        pub mod connection;
    }
    pub mod config;
}

pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    utils::config::init();
    let pool = utils::database::connection::get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(routes::user::get_users)
            .service(routes::user::get_user_by_id)
            .service(routes::user::add_user)
            .service(routes::user::delete_user)
            .service(routes::user::patch_user_email)
            .service(routes::user::patch_user_first_name)
            .service(routes::user::patch_user_last_name)
            .service(routes::todo_task::add_task)
            .service(routes::todo_task::get_tasks)
            .service(routes::todo_task::get_task_by_id)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
