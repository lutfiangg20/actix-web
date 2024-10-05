use actix_web::web;
use diesel::{associations::HasTable, query_dsl::methods::{FindDsl, SelectDsl}, RunQueryDsl, SelectableHelper};

use crate::{
    database::establish_connection,
    models::user_model::{NewUser, UpdateUser, User},
    schema::users::{self},
};

pub fn create_user(user: web::Json<NewUser>) {
    let conn = &mut establish_connection();
    let new_post = NewUser {
        name: user.name.to_string(),
        username: user.username.to_string(),
        email: user.email.to_string(),
        age: user.age,
    };

    diesel::insert_into(users::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn get_user() -> Vec<User> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    results
}

pub fn update_user(path: web::Path<i32>,user: web::Json<UpdateUser>) {
    use crate::schema::users::dsl::users;
    let conn = &mut establish_connection();

    let selecteed_id = path.into_inner();

    let update_user = UpdateUser {
        name: user.name.as_ref().map(|s| s.to_string()),
        username: user.username.as_ref().map(|s| s.to_string()),  // This will not be updated
        email:  user.email.as_ref().map(|s| s.to_string()),
        age:  user.age.as_ref().map(|s| s.to_owned()),
    };

     diesel::update(users::table().find(selecteed_id))
            .set(&update_user)
        .execute(conn)
        .expect("Error updating post");
}

pub fn delete_user(path: web::Path<i32>) {
    use crate::schema::users::dsl::users;
    let conn = &mut establish_connection();
    let selecteed_id = path.into_inner();
    diesel::delete(users.find(selecteed_id))
        .execute(conn)
        .expect("Error deleting post");
}

