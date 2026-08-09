use err_tools::{*,traceable::*};
use anyhow::*;
use axum::{
    routing::get,
    Router,
};


async fn hello()->&'static str {
    "hello fools"
}


pub fn run_server()->Result<(),TraceError>{
    println!("Running Server");
    

    let rt = tokio::runtime::Runtime::new().map_err(any_wrap!("Could not start runtime"))?;

    rt.block_on(async {
            let app = Router::new().route("/", get(hello));

            // run our app with hyper, listening globally on port 3000
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            axum::serve(listener, app).await.unwrap();
    });

    Result::<(),TraceError>::Ok(())

}