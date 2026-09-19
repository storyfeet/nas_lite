use diesel::{prelude::*, sqlite::SqliteConnection};
use diesel_async::{
    RunQueryDsl,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
    sync_connection_wrapper::SyncConnectionWrapper,
};
pub type CManager = SyncConnectionWrapper<SqliteConnection>;
pub type CPool = Pool<CManager>;
