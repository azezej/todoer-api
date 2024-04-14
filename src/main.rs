use actix_web::{web, App, HttpServer};

mod models {}
mod routes {
    pub mod echo;
    pub mod hello;
}
mod utils {
    pub mod database {
        pub mod connection;
    }
    pub mod config;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::hello::hello)
            .service(routes::echo::echo)
            .route("/hey", web::get().to(routes::hello::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
