use crate::models::response::ResponseBody;
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error, utoipa::ToResponse, utoipa::ToSchema)]
pub enum ServiceError {
    #[display(fmt = "{error_message}")]
    #[response(content_type = "application/json", example = json!({"message": "Invalid token, please login again", "data": ""}))]
    Unauthorized { error_message: String },

    #[display(fmt = "{error_message}")]
    #[response(content_type = "text/plain", example = "")]
    InternalServerError { error_message: String },

    #[display(fmt = "{error_message}")]
    #[response(content_type = "text/plain", example = "Json deserialize error: invalid type: integer `123`, expected a string at line 3 column 18")]
    BadRequest { error_message: String },

    #[display(fmt = "{error_message}")]
    #[response(content_type = "text/plain", example = "Not found.")]
    NotFound { error_message: String },
}

impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ServiceError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ServiceError::NotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ResponseBody::new(&self.to_string(), String::from("")))
    }
}
