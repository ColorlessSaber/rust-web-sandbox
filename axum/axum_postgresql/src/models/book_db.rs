use axum::{
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub genre: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookDetail {
    pub id_num: String,
    pub title: String,
    pub author: String,
    pub genre: String,
}

// To allow us to use BookDetail as a response from a handler, we need to
// implement the IntoResponse trait. it tells Axum how to handle the object.
impl axum::response::IntoResponse for BookDetail {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}
