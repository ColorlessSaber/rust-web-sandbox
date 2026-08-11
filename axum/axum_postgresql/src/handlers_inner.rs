/*
Handlers in Axum buffer the request body and returns it.

They are an async function that accepts zero or more "extractors" as arguments and
returns something that can be converted into a response. IE, the application logic
lies within the handler and the Axum application are built by routing between handlers.
 */
use axum::http::StatusCode;
use log::error;

pub mod handlers_for_book_db;

#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest,
    InternalError,
}

impl axum::response::IntoResponse for HandlerError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            HandlerError::BadRequest => {
                (StatusCode::BAD_REQUEST, "An error occurred while processing the request. Please try again later.")
            },
            HandlerError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred")
            },
        };

        body.into_response()
    }
}

pub async fn hello_world() -> String {
    "Hello, World!\n".to_owned()
}

pub async fn trip_internal_error() -> Result<(), HandlerError> {
    error!("An internal error occurred; this is intentional.");
    Err(HandlerError::InternalError)
}