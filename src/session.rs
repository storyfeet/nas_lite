use crate::common_types as CT;
use crate::db_util as DB;
use crate::errors::trace_ok;
use crate::models as MD;
use crate::schema as SCH;
use anyhow::*;
use chrono::Utc;
use diesel::{prelude::*, sqlite::SqliteConnection};
use diesel_async::{
    RunQueryDsl,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
    sync_connection_wrapper::SyncConnectionWrapper,
};
use err_tools::traceable::TraceError;
use err_tools::{traceable::*, *};
use rand::distr::{Alphanumeric, SampleString};

fn new_token_string(len: usize) -> String {
    let alpha = Alphanumeric::default();
    alpha.sample_string(&mut rand::rng(), len)
}

pub fn new_session(user_id: i64) -> MD::NewSession {
    let expires = Utc::now().naive_utc() + chrono::Duration::hours(1);
    MD::NewSession {
        token: new_token_string(32),
        token_pass: new_token_string(32),
        user_id,
        expires,
    }
}

pub async fn create_user_session(
    user_id: i64,
    cpool: &mut DB::CPool,
) -> Result<CT::SessionData, TraceError> {
    let mut con = cpool
        .get()
        .await
        .map_err(any_wrap!("Could not access connection pool"))?;

    let new_session = crate::session::new_session(user_id);

    diesel::insert_into(crate::schema::sessions::table)
        .values(&new_session)
        .execute(&mut con)
        .await
        .map_err(any_wrap!("Could not insert new session"))?;

    return trace_ok(CT::SessionData {
        token: new_session.token,
        token_pass: new_session.token_pass,
        expires: new_session.expires.and_utc().to_rfc3339(),
    });
}

pub async fn check_session(
    token: CT::Token,
    cpool: &mut DB::CPool,
) -> Result<Option<MD::LoadedSession>, TraceError> {
    let mut con = cpool
        .get()
        .await
        .map_err(any_wrap!("Could not access connection pool"))?;
    use SCH::sessions::columns as SCOL;

    SCH::sessions::dsl::sessions
        .filter(SCOL::token.eq(&token.token))
        .filter(SCOL::token_pass.eq(&token.token_pass))
        .filter(SCOL::expires.ge(Utc::now().naive_utc()))
        .inner_join(SCH::users::table)
        .limit(5)
        .select(MD::LoadedSession::as_select())
        .first(&mut con)
        .await
        .optional()
        .map_err(any_wrap!("Could not session by key {}", &token.token))
}
