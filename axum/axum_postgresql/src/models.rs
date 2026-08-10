use thiserror::Error;

pub mod book_db;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}