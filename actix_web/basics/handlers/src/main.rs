// ~~~ Basics of a Handler ~~~
/*
A request handler is an async function that accepts zero or more parameters that can be extracted
from a request (impl FromRequest) and returns a type that can be converted into an HttpResponse
(impl Responder).

There are two stages for a request handler:
1. The handler object is called, returning any object that implements the responder trait.
2. respond_to() is called on the returned object, converting itself to a HttpResponse or Error.

Actix Web provides Responder implementation by default to some standard types, like: &'static str,
Sting etc.

Examples of valid handlers:
```
async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

async fn index(_req: HttpRequest) -> String {
    "Hello world!".to_owned()
}
```

The impl Responder can be used for the function return. it also helps with handling more
complex types.
```
async fn index(_req: HttpRequest) -> impl Responder {
    web::Bytes::from_static(b"Hello world!")
}

async fn index(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    ...
}
```
 */

// ~~~ Response with custom type ~~~
/*
Any custom type that is returned from a handler needs to implement the Responder trait.
 */
// use actix_web::{body::BoxBody, http::header::ContentType, App, HttpRequest, HttpResponse, HttpServer, Responder, web};
// use serde::Serialize;
//
// #[derive(Serialize)]
// struct MyObj {
//     name: &'static str,
// }
//
// // implement the Responder trait to MyObj
// impl Responder for MyObj {
//     type Body = BoxBody;
//
//     fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
//         let body = serde_json::to_string(&self).unwrap();
//
//         // Creating the responder. The content type is defined and the data/body is passed
//         // into it.
//         HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .body(body)
//     }
// }
//
// async fn index() -> impl Responder {
//     MyObj { name: "<" }
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(
//         || {
//             App::new()
//                 .route("/", web::get().to(index))
//         }
//     )
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
/*
To test the code, run the following cURL command:
```
curl -X GET http://127.0.0.1:8080/
```
 */

// ~~~ Steaming response body ~~~
/*
To generate the response body asynchronously the steam trait needs to be implemented.
(Stream<Item = Result<Bytes, Error>>)
 */
use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use futures::{future::ok, stream::once};

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(stream))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
/*
Run the cURL command to test code:
```
curl -X GET http://127.0.0.1:8080/stream
```
 */

// ~~~ Different return types (Either) ~~~
/*
There will be cases where you need different types of a response. An example is: error check
and return errors, return async responses, or any result that requires two different types.
The Actix Web Either type can be used to combine two different responder types into one.

```
use actix_web::{Either, Error, HttpResponse};

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

async fn index() -> RegisterResult {
    if is_a_variant() { // the logic to test which response to give
        // respond with Left variant
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        // respond with Right variant
        Either::Right(Ok("Hello!"))
    }
}
```
 */