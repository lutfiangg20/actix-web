use crate::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
pub mod user_model;

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
