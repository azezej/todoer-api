use actix_web::{http::header::ContentType, HttpResponse};
use serde_json::Value;

pub fn throw_response_created(response_body: Value) -> HttpResponse {
    HttpResponse::Created().content_type(ContentType::json()).json(response_body)
}

pub fn throw_response_ok(response_body: Value) -> HttpResponse {
    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(response_body)
}

pub fn throw_response_error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}