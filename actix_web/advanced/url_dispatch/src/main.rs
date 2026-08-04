// ~~~ Basics of URL Dispatch ~~~
/*
URL dispatch provides a simple way for mapping URLs to handler code using a simple pattern matching
language. A particular handler object is invoked if one of the patterns matches the path information
associated with a request.
 */

// ~~~ Resource configuration ~~~
/*
The act of adding a new resources to an application is known as Resource Configuration.
Each resource has a name and each resource acts as an identifier to be used for URL generation.
Also, each resource has a pattern mean to match against the PATH portion of a URL.

The App::route() method provides a simple way of registering routes. This method adds a single route
to the application routing table. It accepts a path pattern, HTTP method, and a handler function.
Now the route() method can be called multiple times for the same path. It should be noted that
the first match will be used unless the HTTP methods or guards are different.
 */
// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
//
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!\n")
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(index))
//             .route("/user", web::post().to(index))
//     })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
/*
To test code use the following cURL commands.
```
curl -X GET http://127.0.0.1:8080/
```
```
curl -X POST http://127.0.0.1:8080/user
```
 */

/*
To access the complete resource configuration, the App::service() method needs to be used.
This method adds a single resource to the application routing table and accepts: path pattern,
guards, and more or more routes.
 */
// use actix_web::{guard, web, App, HttpResponse, HttpServer};
//
// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("Hello world 2\n")
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(web::resource("/prefix").to(index))
//             .service(
//                 web::resource("/user/{name}")
//                     .name("user_detail")
//                     .guard(guard::Header("content-type", "application/json"))
//                     .route(web::get().to(HttpResponse::Ok))
//                     .route(web::post().to(HttpResponse::Ok)),
//
//             )
//     })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
/*
**NOTE** If a resource does not contain any route or does not have any matching routes, it returns
NOT FOUND HTTP response.
 */

// ~~~ Configuring a Route ~~~
/*
Each route has a set of guards and a handler, and each resource contains a set of routes.
The Resource::route() method can be used to create a new Route instance. By default, the route
does not contain any guards, so it will match all requests and the default handler is HttpNotFound.
It should be noted a Route can contain any number of guards but only have one handler.

```
App::new().service(
    web::resource("/path").route(
        web::route()
            .guard(guard::Get())
            .guard(guard::Header("content-type", "text/plain"))
            .to(HttpResponse::Ok),
    ),
)
```
 */

// ~~~ Route Matching ~~~
/*
The route matching process is down in the order that the routes were declared via App::services().
If resource can not be found, the default resource is used as teh matched resource.
All route guards associated with a route declaration must be true for the route configuration to be
used for a given request during a check. If one guard returns false then the route is skipped and
the route matching continues.
A NOT FOUND response is returned if none of the routes matches.
 */


// ~~~ Resource pattern syntax ~~~
/*
The pattern matching Actix uses is straight forward.
* Each route starts with a slash, if there isn't one then an implicit one is inserted.
* A variable part is specified in the form {identifier}
 */

fn main() {

}