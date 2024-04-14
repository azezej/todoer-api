use actix_web::{web, App, HttpServer};

mod config;
mod routes {
    pub(crate) mod echo;
    pub(crate) mod hello;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::print_config();
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
