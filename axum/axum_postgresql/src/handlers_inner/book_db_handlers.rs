use crate::AppState;
use crate::models::book_db::{Book, BookDetail, BookIndex};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use log::error;

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

pub async fn get_book_detail_by_id(
    State(app_state): State<AppState>,
    Json(payload): Json<BookIndex>
) -> Result<BookDetail, (StatusCode, String)> {
    let book_database = &*app_state.book_db;
    let book = book_database.get_entry(payload.id_num).await;

    match book {
        Ok(book) => {
            Ok(book)
        },
        Err(err) => {
            error!("Error getting book: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get book".to_owned()))
        }
    }
}

pub async fn delete_book_entry(
    State(app_state): State<AppState>,
    Json(payload): Json<BookIndex>,
) -> Result<String, (StatusCode, String)> {
    let book_database = &*app_state.book_db;
    let book = book_database.delete(payload.id_num).await;

    match book {
        Ok(_) => {
            Ok(String::from("Book successfully deleted"))
        },
        Err(err) => {
            error!("Error deleting book: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to deleting book".to_owned()))
        }
    }
}
