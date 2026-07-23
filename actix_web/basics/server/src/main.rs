// ~~~ Basic HttpServer Setup ~~~
/*
To create a server in Actix Web, you use the HttpServer. It's responsible for serving HTTP requests.
The HttpServer takes an application factory as a parameter, and it must have Send and Sync boundaries.
 */
// use actix_web::{App, web, HttpServer, HttpResponse};
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().route("/", web::get().to(HttpResponse::Ok))
//     })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
//     /*
//     **NOTE**
//     1. If the sucket is being used then the application will fail.
//     2. The socket address can be either a tuple or a string. Example of string: 127.0.0.1:8080
//      */
// }


// ~~~ Multi-Threading ~~~
/*
By default, the number of HTTP workers is equal to the number of physical CPUs in the system; IE,
how many cores the computer CPU has the application is running on.
 */
// use actix_web::{ App, HttpServer, web, HttpResponse, };
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(|| async {HttpResponse::Ok().body("Hello world!")}))
//     })
//         .workers(4) // this method overrides the default number of workers.
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }
/*
Each worker will receive a separate application instance to handle requests. Now the application
state is not shared between the threads, and the handlers are free to manipulate their cope of the
state with no concurrency concerns.

Arc/Data can be used to share state between worker threads. However, performance costs can
occur due to locking the state for modifications. Using read/write locks instead of mutexes can
be used to achieve non-exclusive locking.

To avoid handlers locking the current thread when doing long non-cpu bound operations (I/O,
database operations, etc) should be expressed as futures or asynchronous functions.
```
async fn my_handler() -> impl Responder {
    tokio::time::sleep(Duration::from_secs(5)).await;
    "response"
}
```

Same thing can happen when a handler receives an argument which implements FromRequest, and it
blocks the current thread. This is why implementing asynchronously where needed is important.
 */


// ~~~ TLS / HTTPS ~~~
/*
Actix Web supports two TLS implementations: rustls and openssl.
The rustls crate feature is for rustls integration and openssl is for openssl integration.

**NOTE** TLS stands for Transport Layer Security. It's a security protocol used in web development
to protect communication between web browser and a server. HTTPs sites use it for privacy,
integrity, and authentication.
 */
// use actix_web::{get, App, HttpRequest, HttpServer, Responder};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
//
// #[get("/")]
// async fn index(_reg: HttpRequest) -> impl Responder {
//     "Welcome!"
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
//     builder
//         .set_private_key_file("key.pem", SslFiletype::PEM)?;
//     builder.set_certificate_chain_file("cert.pem")?;
//
//     HttpServer::new(|| App::new().service(index))
//         .bind_openssl("127.0.0.1:8080", builder)?
//         .run()
//         .await
// }
/*
To create a key.pem and cert.pem use the command. FILL IN YOUR OWN SUBJECT.
```
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
    -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
```

To remove the password, then copy nopass.pem to key.pem
```
openssl rsa -in key.pem -out nopass.pem
```
 */

// ~~~ Keep-Alive ~~~
/*
There are times you want to keep a connection open for subsequent requests or completely disable the
connection. In Actix Web there are three ways to keep a connection open for
a define duration and two ways to disable keep-alive:
* Duration::from_secs(75) or KeepAlive::Timeout(75); Both enables 75 second keep-alive timer.
* KeepAlive::Os; uses the OS keep-alive
* None or KeepAlive::Disabled; disable keep-alive
 */
use actix_web::{http::KeepAlive, App, HttpServer};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set keep-alive to 75 seconds
    let _one = HttpServer::new(App::new).keep_alive(Duration::from_secs(75));

    // Use OS's keep-alive (usually quite long)
    let _two = HttpServer::new(App::new).keep_alive(KeepAlive::Os);

    // Disable keep-alive
    let _three = HttpServer::new(App::new).keep_alive(None);

    Ok(())
}
/*
The first keep-alive option is enabled HTTP/1.1 requests if the response does not explicitly
disallow it by setting the connection type to 'close' or 'Upgrade.'

To force close a connection use force_close() method on HttpResponseBuilder.
**NOTE** This example the keep-alive is off for HTTP/1.0 and is on for HTTP/1.1 and HTTP/2.0
```
use actix_web::{http, HttpRequest, HttpResponse};

async fn index(_req: HttpRequest) -> HttpResponse {
    let mut resp = HttpResponse::Ok()
        .force_close() // <- Close connection on HttpResponseBuilder
        .finish();

    // Alternatively close connection on the HttpResponse struct
    resp.head_mut().set_connection_type(http::ConnectionType::Close);

    resp
}
```
 */

// ~~~ Graceful shutdown ~~~
/*
There is a way to have HttpServer gracefully shutdown. When HttpServer receives a spot signal
all workers have a specific amount of time to finish their tasks. The workers that don't finish
their tasks after the timeout are forced-dropped.

HttpServer's default shutdown timeout is 30 seconds. This value can be changed using the
HttpServer::shutdown_timeout() method. It will be noted that HttpServer supports other commands:
* CTRL-C, which is available on all OSes.
* On Unix
* * SIGNIT - Force shutdown workers
* * SIGTERM - Graceful shutdown workers
* * SIGQUIT - Force shutdown workers.
These commands can be disabled using the HttpServer::disable_signals() method.
 */