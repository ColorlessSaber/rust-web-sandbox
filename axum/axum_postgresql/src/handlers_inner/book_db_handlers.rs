use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use axum::response::IntoResponse;
use log::error;
use crate::models::book_db::{Book, BookDetail};
use crate::AppState;

pub async fn create_new_book_entry(
    State(app_state): State<AppState>,
    Json(payload): Json<Book>,
) -> Result<BookDetail, (StatusCode, String)> {
    let book_database = &*app_state.book_db;
    let book = book_database.create(payload).await;
    match book {
        Ok(book) => {
            Ok(book)
        },
        Err(err) => {
            error!("Error creating new book: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create book".to_owned()))
        }
    }
}