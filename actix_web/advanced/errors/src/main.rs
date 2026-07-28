// ~~~ Errors 101 ~~~
/*
Actix Web has its own Error type and ResponseError trait for handling web handlers' error.
Thus, when a handler returns a Result it uses the Actix Web Error.

The ResponseError can handle some of the common non-actix errors, like io::Error which is
concerted into an HttpInternalServerError
 */

// ~~~ An Example of a Custom Error Response ~~~
/*
Below is an example of an implementation the ResponseError trait.
 */
// use actix_web::{error, Result};
// use derive_more::derive::{Display, Error};
//
// #[derive(Debug, Display, Error)]
// #[display("my error: {name}")]
// struct MyError {
//     name: &'static str,
// }
//
// // Use default implementation for error_response() method
// impl error::ResponseError for MyError {}
//
// async fn index() -> Result<&'static str, MyError> {
//     Err(MyError { name: "test" })
// }
/*
By default, the error_response() will render a 500 (internal server error) code. This will happen
with the code above.

To override the error_response() default error code, see the example code below.
 */
// use actix_web::{error, get, http::{header::ContentType, StatusCode}, App, HttpResponse, HttpResponseBuilder};
// use derive_more::derive::{Display, Error};
//
// #[derive(Debug, Display, Error)]
// enum MyError {
//     #[display("internal error")]
//     InternalError,
//
//     #[display("bad request")]
//     BadClientData,
//
//     #[display("timeout")]
//     Timeout,
// }
//
// impl error::ResponseError for MyError {
//     fn status_code(&self) -> StatusCode {
//         match *self { // dereference the enum to avoid matching against reference.
//             MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//             MyError::BadClientData => StatusCode::BAD_REQUEST,
//             MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
//         }
//     }
//
//     fn error_response(&self) -> HttpResponse {
//         HttpResponseBuilder::new(self.status_code())
//             .insert_header(ContentType::html())
//             .body(self.to_string())
//     }
// }
//
// #[get("/")]
// async fn index() -> Result<&'static str, MyError> {
//     Err(MyError::BadClientData)
// }

// ~~~ Error helpers ~~~
/*
There is a set of error helper functions provided by Actix Web that are useful for generating
specific HTTP error codes from other errors.

Check the API documentation for actix-web's error module for full list of available error helpers.
 */
// use actix_web::{error, get,};
//
// #[derive(Debug)]
// struct MyError {
//     name: &'static str,
// }
//
// #[get("/")]
// async fn index() -> actix_web::Result<String> {
//     let result = Err(MyError { name: "test error" });
//
//     result.map_err(|err| error::ErrorBadRequest(err.name))
// }

// ~~~ Error logging ~~~
/*
All errors in Actix are logged at WARN log level. If the application's log level is set to DEBUG
and RUST_BACKTRACE is enabled, the backtrace is also logged.
 */

// ~~~ Recommended practices in error handling ~~~
/*
The recommended practice is to divide the errors of an application into two groups:
* the errors the user is allowed to see
* the errors the user is not allowed to see

Also, in some cases it best to hide the specifics of what caused the error from the user. In those
cases it best to map the errors to a generic error suitable for user consumption.
 */
// use actix_web::{error, get, http::{header::ContentType, StatusCode}, App, HttpResponse, HttpServer};
// use derive_more::derive::{Display, Error};
//
// #[derive(Debug, Display, Error)]
// enum UserError {
//     #[display("An internal error occurred. Please try again later.")]
//     InternalError,
// }
//
// impl error::ResponseError for UserError {
//     fn status_code(&self) -> StatusCode {
//         match *self {
//             UserError::InternalError => StatusCode::BAD_REQUEST,
//         }
//     }
//
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::html())
//             .body(self.to_string())
//     }
// }
//
// #[get("/")]
// async fn index() -> Result<&'static str, UserError> {
//     do_thing_that_fails().map_err(|_e| UserError::InternalError)?;
//
//     Ok("Success!")
// }

// ~~~ Error Logging ~~~
/*
Below is an example of logging events
 */
use actix_web::{error, get, middleware::Logger, App, HttpResponse, HttpServer, Result};
use derive_more::derive::{Display, Error};
use log::info;

#[derive(Debug, Display, Error)]
#[display("My Error: {name}")]
pub struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<&'static str, MyError> {
    let error = MyError { name: "test error" };
    info!("{}", error);
    Err(error)
}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

/*
To test code, use the following cURL command:
```
curl -X GET http://127.0.0.1:8080/
```
 */
