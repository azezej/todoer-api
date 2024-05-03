extern crate diesel;

use actix_web::{web::Data, App, HttpServer};
use scopes::{swagger, todo_list_scope, todo_task_scope, user_scope};

pub mod constants;
pub mod scopes;
mod models {
    pub mod dto {
        pub mod todo_list;
        pub mod todo_task;
        pub mod user;
    }
    pub mod utils {
        pub mod tailored_response;
    }
    pub mod login_history;
    pub mod response;
    pub mod todo_list;
    pub mod todo_task;
    pub mod user;
    pub mod user_token;
}
mod routes {
    pub mod todo_list;
    pub mod todo_task;
    pub mod user;
}
mod utils {
    pub mod database {
        pub mod connection;
    }
    pub mod config;
}
mod service {
    pub mod todo_list {
        pub mod todo_list_service;
    }
    pub mod todo_task {
        pub mod todo_task_service;
    }
    pub mod user {
        pub mod user_service;
    }
}

pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    utils::config::init();
    let pool = utils::database::connection::get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(user_scope())
            .service(todo_task_scope())
            .service(todo_list_scope())
            .service(swagger())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
