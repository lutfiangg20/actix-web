use crate::models::*;
use crate::services::user_service::{create_user, delete_user, get_user, update_user};
use actix_web::{web, HttpResponse, Responder};
use user_model::{NewUser, UpdateUser};

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

    pub async fn update(id: web::Path<i32>,user: web::Json<UpdateUser>) -> impl Responder {
        update_user(id, user);
        HttpResponse::Ok().body(format!("Update user "))
    }

    pub async fn delete(id: web::Path<i32>) -> impl Responder {
        delete_user(id);
        HttpResponse::Ok().body(format!("Delete user "))
    }
}
