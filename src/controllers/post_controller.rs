use crate::models::*;
use crate::services::post_service::{create_post, get_posts};
use actix_web::{web, HttpResponse, Responder};

pub struct PostController {}

impl PostController {
    pub async fn get_post() -> impl Responder {
        let results = get_posts();
        HttpResponse::Ok().json(results)
    }

    pub async fn create_post(user: web::Json<Post>) -> impl Responder {
        create_post(user);
        HttpResponse::Ok().body("Post created")
    }
}
