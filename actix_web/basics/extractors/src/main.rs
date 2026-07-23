// ~~~ Basics of Extractors ~~~
/*
In Actix Web, an Extractor is a type-safe request information access. They can be an argument to a
handler function. One handler function can support up to 12 extractors.

Usually the argument position does not matter. But if the extractor takes the body (IE, reads
any bytes from a request body stream) then only the first extractor will succeed. Fallback can be
setup for an extractor. For example, "read body as JSON or just give the raw byles if fails" is done
like so: Either<Json<T>, Bytes>.

There are specific use cases where request bodies need reading twice are supported.
* For body (any extractor) + it's hash/digest, see the actix-hash crate.
* For body (any extractor) + custom request signature scheme, see RequestSignature extractor
from crate actix-web-lab.

Example of two positional dynamic path segments and a JSON body.
```
async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}
```
 */

// ~~~ Path ~~~
/*
The web::Path provides information that is extracted from request's path. Any variable segment from
the path can be deserialized. The parts of the path that are extractable are called "dynamic segments"
and are marked with curly braces.
 */
// use actix_web::{ web, App, get, HttpServer, Result, HttpRequest};
// use serde::Deserialize;
//
// // Extract path from "users/{user_id}/{friend}" url
// // {user_id} - deserializes to an u32
// // {friend} - deserializes to a string
// #[get("/users/{user_id}/{friend}")]
// async fn index(path: web::Path<(u32, String)>) -> Result<String> {
//     let (user_id, friend) = path.into_inner();
//     Ok(format!("Welcome {}, user_id {}!", friend, user_id))
// }
//
// // A struct can be used to hold the path information, if it implements the Deserialize from serde
// // crate. (Make sure the serde feature "derive" is enabled).
// #[derive(Deserialize)]
// struct PetInfo {
//     pet_id: u32,
//     pet_name: String,
// }
//
// #[get("/pets/{pet_id}/{pet_name}")]
// async fn pets(pet: web::Path<PetInfo>) -> Result<String> {
//     Ok(format!(
//         "Your pet {} (pet id {}) is ready for its bath.",
//         pet.pet_name, pet.pet_id
//     ))
// }
//
// // For a non-type-safe alternative, you can query the request for the path parameters by the name
// // within the handler
// #[get("/anime/{anime_id}/{anime_name}")]
// async fn anime(req: HttpRequest) -> Result<String> {
//     let anime_name: String = req.match_info().query("anime_name").parse()?;
//     let anime_id: i32 = req.match_info().query("anime_id").parse().unwrap();
//
//     Ok(format!("You selected anime {}. Anime ID: {}", anime_name, anime_id))
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new()
//         .service(index)
//         .service(pets)
//         .service(anime)
//     )
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
/*
Use the cURL command to test "user" url:
```
curl -X GET http://127.0.0.1:8080/users/12/michael
```

use the cURL command to test "pets" url:
```
curl -X GET http://127.0.0.1:8080/pets/7/spot
```

use the cURL command to test "anime" url:
```
curl -X GET http://127.0.0.1:8080/anime/345/Slayers
```
 */

// ~~~ Query ~~~
/*
Actix Web has a Query<T> type that provides extraction functionality for request's query parameters.
it should be noted the Query<T> uses the serde_urlencoded crate.
 */
// use actix_web::{ web, get, App, Responder, HttpServer};
// use serde::Deserialize;
//
// #[derive(Deserialize)]
// struct AuthCreds {
//     username: String,
//     password: String,
// }
//
// #[get("/")]
// async fn index(info: web::Query<AuthCreds>) -> impl Responder {
//     format!("User {}, password {} \n", info.username, info.password)
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

/*
To test this functionality, use the following cURL command:
```
curl -X GET "http://127.0.0.1:8080/?username=legends2000&password=Sonic1234_Pass"
```
 */

// ~~~ Json ~~~
/*
The Json<T> deserialize the body of the request into a struct.
 */
use actix_web::{web, App, post, error, HttpServer, Result, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

// Some extractors can be configured for the extraction process. This is done by
// passing a configuration object to the resource's .app_data() method.
// For json extractor it returns JsonConfig. This can be used to maximum size of the JSON payload
// and custom error handler.

async fn json_index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create a custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });

        App::new()
            .service(submit)
            .service(
                web::resource("/json")
                    .app_data(json_config)
                    .route(web::post().to(json_index))
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

/*
To test /submit use the following cURL command:
```
curl -X POST http://127.0.0.1:8080/submit -H "Content-Type: application/json" -d '{"username": "sonic312mph"}'
```

To test /json use the following cURL command:
```
curl -X POST http://127.0.0.1:8080/json -H "Content-Type: application/json" -d '{"username": "sonic312mph"}'
```
 */

// ~~~ URL-Encoded Forms ~~~