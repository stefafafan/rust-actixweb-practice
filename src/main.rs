// https://actix.rs/docs/getting-started/
// https://actix.rs/docs/url-dispatch/
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    username: String,
    name: String,
}

async fn user_detail() -> HttpResponse {
    let user = User {
        id: 12345,
        username: "stefafafan".to_string(),
        name: "すてにゃん".to_string(),
    };
    let serialized = serde_json::to_string(&user).unwrap();
    let deserialized: User = serde_json::from_str(&serialized).unwrap();
    HttpResponse::Ok().body(format!("{:?}", deserialized))
}

async fn user_detail_json() -> HttpResponse {
    let user = User {
        id: 12345,
        username: "stefafafan".to_string(),
        name: "すてにゃん".to_string(),
    };
    let serialized = serde_json::to_string(&user).unwrap();
    HttpResponse::Ok().body(format!("{}", serialized))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/users")
                .route("/show", web::get().to(show_users))
                .route("/show/{id}.json", web::get().to(user_detail_json))
                .route("/show/{id}", web::get().to(user_detail)),
        )
        .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
