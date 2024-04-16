extern crate diesel;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

mod models {
    pub mod error;
    pub mod user;
}
mod routes {
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
            .route("/users", web::get().to(routes::user::get_users))
            .route("/users/{id}", web::get().to(routes::user::get_user_by_id))
            .route("/users", web::post().to(routes::user::add_user))
            .route("/users/{id}", web::delete().to(routes::user::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
