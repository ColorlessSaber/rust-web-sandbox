use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::book_db::{Book, BookDetail};
use crate::models::DBError;

// A trait with shared
#[async_trait]
pub trait BookDbCrud {
    async fn create(&self, query_info: Book) -> Result<BookDetail, DBError>;
    async fn delete(&self, index_num: String) -> Result<(), DBError>;
    async fn get_entry(&self) -> Result<BookDetail, DBError>;
}

// struct to hold access to the database
pub struct BookDbImpl {
    db: PgPool,
}

impl BookDbImpl {
    pub fn new(db: PgPool) -> Self {
        Self {db}
    }
}

#[async_trait]
impl BookDbCrud for BookDbImpl {
    async fn create(&self, query_info: Book) -> Result<BookDetail, DBError> {
        let query_result = sqlx::query!( // this is a "reminder" warning; will work if sqlx setup is correct
            r#"
            INSERT INTO book_db (title, author, genre)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            query_info.title,
            query_info.author,
            query_info.genre
        )
            .fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(BookDetail {
            id_num: query_result.id.to_string(),
            title: query_result.title.unwrap(),
            author: query_result.author.unwrap(),
            genre: query_result.genre.unwrap(),
        })
    }

    async fn delete(&self, index_num: String) -> Result<(), DBError> {
        todo!()
    }

    async fn get_entry(&self) -> Result<BookDetail, DBError> {
        todo!()
    }
}