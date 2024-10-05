use crate::database::establish_connection;
use crate::models::*;
use actix_web::{get, post, web::Path, HttpResponse, Responder};
use diesel::prelude::*;
use post_model::Post;

#[get("/")]
pub async fn hello() -> impl Responder {
    /* use crate::schema::posts::dsl::posts; */
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
    HttpResponse::Ok().body("Hello world!")
}

#[post("/create")]
pub async fn create(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn coba_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn get_by_id(path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(id)
}
