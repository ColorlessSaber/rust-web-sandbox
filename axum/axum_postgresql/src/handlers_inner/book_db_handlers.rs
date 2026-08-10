use crate::models::book_db::{Book, BookDetail};
use crate::handlers_inner::HandlerError;

pub async fn create_new_book_entry(
    book: Book,
) -> Result<BookDetail, HandlerError> {
    let foo = BookDetail {
        id_num: "1".to_owned(),
        title: "Can't spell treason without tea".to_owned(),
        author: "Rebecca Thorne".to_owned(),
        genre: "fantasy".to_owned()
    };
    Ok(foo)
}