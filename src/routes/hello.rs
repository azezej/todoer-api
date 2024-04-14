use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub(crate) async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub(crate) async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
