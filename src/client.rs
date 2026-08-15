use anyhow::*;
use err_tools::{traceable::*, *};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ClientArgs {
    user_name: String,
    password: String,
    url: Option<String>,
}

pub fn run_client(client_args: ClientArgs) -> Result<(), TraceError> {
    let url = client_args
        .url
        .or_else(|| dotenvy::var("CLIENT_URL").ok())
        .ok_or(err_at!("No URL to load from"))?;

    let rt = tokio::runtime::Runtime::new().map_err(any_wrap!("Could not start runtime"))?;

    rt.block_on(async {
        // Open session - get tokens
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/login", url))
            .send()
            .await
            .expect("Got no response from server");

        println!("Response : {:?}", &res);

        let bytes = res.bytes().await.expect("No Bytes in response");

        let body = std::str::from_utf8(&bytes).expect("Could not convert response to string");

        println!("Response body is : {}", body);

        // Keep checking for updates -- main loop
    });

    Result::<(), TraceError>::Ok(())
}
