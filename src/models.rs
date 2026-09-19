use crate::common_types as CT;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub id: i64,
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
    pub user_id: i64,
    pub expires: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: i64,
    pub file_type: CT::FileType,
    pub file_name: String,
    pub file_size: i64,
    pub chunk_size: i64,
    pub chunks_loaded: i32,
    pub file_hash: String,
    pub completed: Option<NaiveDateTime>,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub deleted: Option<NaiveDateTime>,
}
