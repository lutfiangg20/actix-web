use actix_web::web;
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};

use crate::{
    database::establish_connection,
    models::user_model::{NewUser, User},
    schema::users,
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
