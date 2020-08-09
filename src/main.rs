// https://actix.rs/docs/getting-started/
// https://actix.rs/docs/url-dispatch/
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.0))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/users")
                .route("/show", web::get().to(show_users))
                .route("/show/{id}", web::get().to(user_detail)),
        )
        .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
