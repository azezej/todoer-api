extern crate diesel;

use actix_web::{web::Data, App, HttpServer};
use scopes::{todo_task_scope, user_scope};

mod constants;
mod error;
mod models;
mod routes;
mod schema;
mod scopes;
mod service;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    utils::config::init();
    let pool = utils::database_connection::get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(user_scope())
            .service(todo_task_scope())
        //    .service(todo_list_scope())
        //            .service(swagger())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
