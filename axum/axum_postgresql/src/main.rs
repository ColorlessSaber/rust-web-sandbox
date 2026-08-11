extern crate log;
extern crate pretty_env_logger;

use crate::handlers_inner::{
    handlers_for_book_db::*
    ,
    *};
use axum::{
    Router,
    routing::{delete, get, post},
    serve,
};
use database_queries::queries_for_book_db::{BookDbCrud, BookDbImpl};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

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

    // Set up the AppState of the Axum application
    let book_db = Arc::new(BookDbImpl::new(pool));
    let app_state = AppState { book_db };

    // Set up the Axum application
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/trip_internal_error", get(trip_internal_error))
        .route("/new_book", post(create_new_book_entry))
        .route("/get_book", get(get_book_detail_by_id))
        .route("/delete_book", delete(delete_book_entry))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(
        &std::env::var("SERVER_PORT").expect("SERVER_PORT not set")
    )
        .await
        .unwrap();

    serve::serve(listener, app).await.unwrap();
}

/*

To test /hello use the cURL command:
```
curl -X GET http://127.0.0.1:8000/hello
```

To test /trip_internal_error use the cURL command:
```
curl -X GET "http://127.0.0.1:8000/trip_internal_error"
```

To test /new_book use the cURL command:
```
curl -X POST "http://127.0.0.1:8000/new_book" \
--json '{"title": "cant spell treason without tea", "author": "Rebecca thorne", "genre": "fantasy"}'
```
it will return what was just submitted with the id number it has in the SQL database.

To test /get_book use the cURL command (replace <index num> with an index number)
```
curl -X GET "http://127.0.0.1:8000/get_book" --json '{"id_num": <index num>}'
```

To test /delete_book use the cURL command (replace <index num> with an index number)
```
curl -X DELETE "http://127.0.0.1:8000/delete_book" --json '{"id_num": <index num>}'
```
 */