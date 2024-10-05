use actix_web::web;
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};

use crate::{
    database::establish_connection,
    models::{NewPost, Post},
    schema::posts,
};

pub fn create_post(user: web::Json<Post>) {
    let conn = &mut establish_connection();
    let new_post = NewPost {
        title: user.title.as_str(),
        body: user.body.as_str(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn get_posts() -> Vec<Post> {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    results
}
