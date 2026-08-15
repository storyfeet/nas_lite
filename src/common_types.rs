use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Deserialize, Clone, Serialize)]
#[structopt()]
pub struct UserPassword {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UserPassword2 {
    pub name: String,
    pub password: String,
}
