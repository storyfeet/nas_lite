use diesel::prelude::*;
//use chrono::{naive::{NaiveDateTime}};

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUser {
    pub user_name: String,
    pub password_hash: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    //pub id:i32,
    pub user_name: String,
    pub password_hash: String,
    //pub created:NaiveDateTime,
    //pub modified:NaiveDateTime,
}
