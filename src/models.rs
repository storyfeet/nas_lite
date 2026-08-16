use chrono::naive::NaiveDateTime;
use diesel::prelude::*;

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
    pub id: i32,
    pub user_name: String,
    pub password_hash: String,
    //pub created:NaiveDateTime,
    //pub modified:NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSession {
    pub token: String,
    pub token_pass: String,
    pub user_id: i32,
    pub expires: NaiveDateTime,
}
