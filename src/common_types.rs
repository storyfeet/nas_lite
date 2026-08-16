/*!
 * This module provides classes to be used by both client server as a means of communication.
 */
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Deserialize, Clone, Serialize)]
#[structopt()]
pub struct UserPassword {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]

pub struct SessionData {
    pub token: String,
    pub token_pass: String,
    pub expires: String,
}
