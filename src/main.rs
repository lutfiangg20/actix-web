use actix_web::{web, App, HttpServer};

mod route;
mod services;
use route::*;
mod controllers;
mod database;
mod models;
mod schema;
use controllers::{post_controller::PostController, user_controller::UserController};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(create)
            .route("/get-post", web::get().to(PostController::get_post))
            .route("/create-post", web::post().to(PostController::create_post))
            .route("/get-user", web::get().to(UserController::get))
            .route("/create-user", web::post().to(UserController::create))
            .route("/hey", web::get().to(manual_hello))
            .route("/post", web::post().to(coba_post))
            .route("/hey/{id}", web::get().to(get_by_id))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
