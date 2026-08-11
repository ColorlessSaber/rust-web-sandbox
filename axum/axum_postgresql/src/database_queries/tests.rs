mod book_db_tests {
    use sqlx::PgPool;
    use dotenvy::from_filename;

    use crate::{
        models::models_for_book_db::*,
        database_queries::queries_for_book_db::*,
    };

    #[tokio::test]
    async fn should_successfully_create_new_book() -> Result<(), String>{
        from_filename(".env.test").expect("Failed to load .env.test file");
        let pool = PgPool::connect(
            &std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        ).await.unwrap();
        let book_db = BookDbImpl::new(pool);

        let result = book_db.create(
            Book {
                title: "Create Unit test".to_owned(),
                author: "Mr. Testy Tester".to_owned(),
                genre: "debugging".to_owned(),
            }
        ).await;

        if result.is_err() {
            return Err(
                format!(
                    "Should have gotten a valid input. {:?}",
                    result.unwrap_err()
                ));
        }

        Ok(())
    }

    #[tokio::test]
    async fn should_get_existing_book_entry() -> Result<(), String> {
        from_filename(".env.test").expect("Failed to load .env.test file");
        let pool = PgPool::connect(
            &std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        ).await.unwrap();

        let book_db = BookDbImpl::new(pool);
        let result = book_db.create(
            Book {
                title: "Grab Unit test".to_owned(),
                author: "Mr. Testy Tester".to_owned(),
                genre: "debugging".to_owned(),
            }
        ).await;

        if result.is_err() {
            return Err(
                format!("Should have successfully created entry. {:?}", result.unwrap_err()
                ));
        }

        let result = book_db.get_entry(result.unwrap().id_num).await;

        if result.is_err() {
            return Err(
                format!("Should have gotten existing book entry {:?}", result.unwrap_err())
            )
        }

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_existing_book_entry() -> Result<(), String> {
        from_filename(".env.test").expect("Failed to load .env.test file");
        let pool = PgPool::connect(
            &std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        ).await.unwrap();

        let book_db = BookDbImpl::new(pool);
        let result = book_db.create(
            Book {
                title: "Delete Unit test".to_owned(),
                author: "Mr. Testy Tester".to_owned(),
                genre: "debugging".to_owned(),
            }
        ).await;

        if result.is_err() {
            return Err(
                format!("Should have successfully created entry. {:?}", result.unwrap_err()
                ));
        }

        let result = book_db.delete(result.unwrap().id_num).await;

        if result.is_err() {
            return Err(
                format!("Should have successfully deleted. {:?}", result.unwrap_err())
            )
        }

        Ok(())
    }

}