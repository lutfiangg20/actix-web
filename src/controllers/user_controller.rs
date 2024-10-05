use crate::models::*;
use crate::services::user_service::{create_user, get_user};
use actix_web::{web, HttpResponse, Responder};
use user_model::NewUser;

pub struct UserController {}

impl UserController {
    pub async fn get() -> impl Responder {
        let results = get_user();
        HttpResponse::Ok().json(results)
    }

    pub async fn create(user: web::Json<NewUser>) -> impl Responder {
        create_user(user);
        HttpResponse::Ok().body("User created")
    }
}
