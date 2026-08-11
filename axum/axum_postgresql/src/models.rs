use thiserror::Error;

pub mod book_db;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
