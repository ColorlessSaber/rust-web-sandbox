use log::error;
use crate::models::book_db::{Book, BookDetail};
use crate::database_queries::book_db_queries::BookDbCrud;
use crate::handlers_inner::HandlerError;

pub async fn create_new_book_entry(
    book: Book,
    book_database: &(dyn BookDbCrud + Sync + Send)
) -> Result<BookDetail, HandlerError> {
    let book = book_database.create(book).await;
    match book {
        Ok(book) => Ok(book),
        Err(err) => {
            error!("Error creating new book: {:?}", err);
            Err(HandlerError::default_internal_error())
        }
    }
}