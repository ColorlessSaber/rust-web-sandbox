extern crate log;
extern crate pretty_env_logger;

use std::sync::Arc;
use axum::{
    routing::{post, },
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use handlers_inner::*;
use database_queries::{ book_db_queries::{BookDbImpl, BookDbCrud}};
use crate::handlers_inner::book_db_handlers::create_new_book_entry;

mod models;
mod handlers_inner;
mod database_queries;

#[derive(Clone)]
struct AppState {
    book_db: Arc<dyn BookDbCrud + Send + Sync>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    // set up the Postgres connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Failed to connect to database");
    let book_db = Arc::new(BookDbImpl::new(pool));

    let app_state = AppState { book_db };
    let app = Router::new()
        .route("/new_book", post(create_new_book_entry))
        .with_state(app_state);

    println!("Hello, world!");
}
