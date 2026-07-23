use actix_web::{web, App, get, post, Responder, HttpResponse, HttpServer};

#[get("/")] // handles the HTTP GET request for URL path "/"
async fn hello() -> impl Responder {
    // Create an instance of HttpResponse with a status code 200 OK
    // and attaches the string "Hello world!" as the body content of this response.
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")] // handles the HTTP POST request for URL path "/echo"
async fn echo(req_body: String) -> impl Responder {
    // Create an instance of HttpResponse with a status code 200 OK
    // and attaches the original request body as the body content of this response.
    HttpResponse::Ok().body(req_body)
}
/*
To test this code, enter the following into a terminal using the cURL CLI
```
curl -X POST http://127.0.0.1:8080/echo --data "Hello from cURL"
```
 */

async fn manual_hello() -> impl Responder {
    // Create an instance of HttpResponse with a status code 200 OK
    // and attaches the string "hey there!" as the body content of this response.
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello) // register HTTP service
            .service(echo)
            .route("/hey", web::get().to(manual_hello)) // configure route path
    })
        .bind(("127.0.0.1", 8080))? // binds to socket address and listens
        .run()
        .await
}
