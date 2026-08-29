use crate::client_config;
use crate::common_types::UserPassword;
use crate::errors::trace_ok;
use anyhow::*;
use err_tools::{traceable::*, *};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ClientArgs {
    config: String,
}

pub fn run_client(client_args: ClientArgs) -> Result<(), TraceError> {
    let rt = tokio::runtime::Runtime::new().map_err(any_wrap!("Could not start runtime"))?;

    rt.block_on(async {
        match run_async_client(client_args).await {
            Result::Ok(v) => println!("Server finished happy"),
            Result::Err(e) => println!("There was an error running the server : {}", e),
        }
        // Open session - get tokens
    });

    Result::<(), TraceError>::Ok(())
}

pub async fn run_async_client(client_args: ClientArgs) -> TraceResult<()> {
    let config = crate::client_config::loadConfig(&client_args.config).await?;
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/login", &config.url))
        .json(&UserPassword {
            name: config.user_name,
            password: config.password,
        })
        .send()
        .await
        .map_err(any_wrap!("Got no response from the server"))?;

    println!("Response : {:?}", &res);

    let bytes = res.bytes().await.expect("No Bytes in response");

    let body = std::str::from_utf8(&bytes).expect("Could not convert response to string");

    println!("Response body is : {}", body);

    // Keep checking for updates -- main loop

    trace_ok(())
}
