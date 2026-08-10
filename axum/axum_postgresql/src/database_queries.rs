use async_trait::async_trait;
pub mod book_db_queries;

use crate::models::DBError;

#[async_trait]
pub trait CrudCmd<T, U> {
    async fn create(&self, object: T) -> Result<U, DBError>;
    async fn delete(&self, index_num: String) -> Result<(), DBError>;
    async fn get_entry(&self) -> Result<U, DBError>;
}