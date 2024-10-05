use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub age: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub username: String,
    pub email: String,
    pub age: i32,
}

#[derive(Insertable, Serialize, Deserialize,AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
}
