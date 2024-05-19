extern crate diesel;

use actix_cors::Cors;
use actix_web::{http::header, web::Data, App, HttpServer};
use scopes::{todo_list_scope, todo_task_scope, user_scope};

mod constants;
mod error;
mod middleware;
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
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_origin("http://localhost:8080")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(middleware::auth_middleware::Authentication)
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(user_scope())
            .service(todo_task_scope())
            .service(todo_list_scope())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
