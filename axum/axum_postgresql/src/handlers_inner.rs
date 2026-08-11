/*
Handlers in Axum buffer the request body and returns it.

They are an async function that accepts zero or more "extractors" as arguments and
returns something that can be converted into a response. IE, the application logic
lies within the handler and the Axum application are built by routing between handlers.
 */
use axum::response::IntoResponse;

pub mod book_db_handlers;

#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

impl HandlerError {
    pub fn default_internal_error() -> Self {
        HandlerError::InternalError("Oops! Ran into problem, please try again.".to_owned())
    }
}

pub async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}