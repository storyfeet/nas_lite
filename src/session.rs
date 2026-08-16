use rand::distr::{Alphanumeric, SampleString};

use crate::models::NewSession;
use chrono::Utc;

fn new_token_string(len: usize) -> String {
    let alpha = Alphanumeric::default();
    alpha.sample_string(&mut rand::rng(), len)
}

pub fn new_session(user_id: i32) -> NewSession {
    let expires = Utc::now().naive_utc() + chrono::Duration::hours(1);
    NewSession {
        token: new_token_string(32),
        token_pass: new_token_string(32),
        user_id,
        expires,
    }
}
