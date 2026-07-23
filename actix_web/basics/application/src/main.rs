// // ~~~ Basic Example ~~~
// /*
// Basic example of an Application
//  */
// use actix_web::{web, App, HttpServer, Responder};
//
// async fn index() -> impl Responder {
//
//     "Hello world!"
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         // The App instance is used for registering routes for resources
//         // and middleware. It also stores application state shared across all handlers within the
//         // same scope.
//         App::new().service(
//             // The web::scope acts as a namespace for all routes. Other words, all routes
//             // associated with it have the same url path prefix.
//             web::scope("/app")
//                 // This route handles requests for 'GET /app/index.html'
//                 .route("/index.html", web::get().to(index))
//             // use the following cURL command to get this resource:
//             // ```
//             // curl -X GET http://127.0.0.1:8080/app/index.html
//             // ```
//         )
//
//         // **NOTE** Any request with the paths /app, /app/, or /app/test would match;
//         // however, the path /application would not match.
//
//     })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

// // ~~~ State Example ~~~
// /*
// Application state is shared within all routes and resources within the same scope.
//  */
// use actix_web::{get, web, App, HttpServer};
//
// struct AppState {
//     app_name: String,
// }
//
// #[get("/")]
// async fn index(data: web::Data<AppState>) -> String {
//     let app_name = &data.app_name; // get app_name
//     format!("Hello {}!", app_name) // response with app_name
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .app_data(web::Data::new(AppState {
//                 app_name: String::from("Actix-web"),
//             }))
//             .service(index)
//     })
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
//     // use the following cURL command:
//     // ```
//     // curl -X GET http://127.0.0.1:8080/
//     // ```
// }


// ~~~ Shared Mutable State Example ~~~
/*
HttpServer accepts an application factory rather than an application instance.
An HttpServer constructs an application instance for each thread. Therefore, application
data must be constructed multiple times. If you want to share data between different threads,
a shareable object should be used.
 */
// use actix_web::{web, App, HttpServer};
// use std::sync::Mutex;
//
// struct AppStateWithCounter {
//     counter: Mutex<i32>, // Mutex is necessary to mutate safely across threads
// }
//
// async fn index(data: web::Data<AppStateWithCounter>) -> String {
//     let mut counter = data.counter.lock().unwrap(); // get counter's MutexGuard
//     *counter += 1; // Access counter inside MutexGuard
//
//     format!("Request number: {}", counter)
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // create the counter outside the HttpServer
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });
//
//     HttpServer::new(move || {
//         // move counter into the closure
//         App::new()
//             .app_data(counter.clone()) // register the created data
//             .route("/", web::get().to(index))
//     })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
//
//     // use cURL command to get counter value. Each time you call it, the counter increases.
//     // ```
//     // curl -X GET http://127.0.0.1:8080/
//     // ```
// }

// ~~~ Using an Application Scope to Compose Applications ~~~
/*
The web::scope() method allows setting a resource group prefix. This scope represents
a resource prefix that will be prepended to all resource patterns added by the resource
configuration. This can be used to help mount a set of routes at a different location than
the original author intended while still maintaining the same resource names.

Example:
```
#[actix_web::main]
async fn main() {
    let scope = web::scope("/users").service(show_users);
    App::new().service(scope);
}
```
Assuming show_users uses '/show' then this setup makes it to where the resource only get
called when URL path matches '/users/show'.
*/

// ~~~ Application guards and virtual hosting ~~~
/*
A guard is a simple function that accepts a request object reference and returns true or false.
It can be anything as long as it implements the Guard trait. Actix Web has several guards,
one of them is 'Host'. This one can be used as a filter based on request header information.
 */
// use actix_web::{guard, web, App, HttpResponse, HttpServer};
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(
//                 // This creates a route for the root path ("/") but only if the request
//                 // comes from www.rust-lang.org. When such a GET request is received, it
//                 // responds with an HTTP 200 OK status and the body "www".
//                 web::scope("/")
//                     .guard(guard::Host("www.rust-lang.org"))
//                     .route("", web::get().to(|| async {HttpResponse::Ok().body("www")})),
//             )
//             .service(
//                 // This is similar to the previous one, but for requests from
//                 // users.rust-lang.org. It responds with an HTTP 200 OK status and
//                 // the body "users".
//                 web::scope("/")
//                     .guard(guard::Host("users.rust-lang.org"))
//                     .route("", web::get().to(|| async { HttpResponse::Ok().body("users")})),
//             )
//             .route("", web::to(HttpResponse::Ok)) // if no specific route matches, this one will be used.
//     })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

// ~~~ Configure ~~~
/*
To make things more simple and reusable, a configure method can be used for both
App and web::Scope. A configure method is useful for moving parts of a configuration to a
different module or library.
For example, some of the resource's configuration could be moved to a
different module.
 */
use actix_web::{web, App, HttpResponse, HttpServer};

// This will be located in one module
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
        .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
        .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// this will be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config) // The "standard" connection
            .service(web::scope("/api").configure(scoped_config))// the "alternate" connection, like api
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Home page") }), // the "default" connection
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
/*
To access the "standard" connection, use the following cURL command:
```
curl -X GET http://127.0.0.1:8080/app
```

To access the "api" connection, use the following cURL command:
```
curl -X GET http://127.0.0.1:8080/api/test
```

To access the "default" connection, use the following cURL command:
```
curl -X GET http://127.0.0.1:8080/
```

If you wish to trigger the HEAD guard, use the following cURL command:
```
curl --head http://127.0.0.1:8080/app
```
 */