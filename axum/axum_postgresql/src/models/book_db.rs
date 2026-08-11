use axum::http::StatusCode;
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

impl axum::response::IntoResponse for BookDetail { //TODO make this return a JSON dump versus itself
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, self).into_response()
    }
}
