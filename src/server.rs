use anyhow::*;
use axum::{Router, extract::Path, routing::get};
use diesel::sqlite::SqliteConnection;
use diesel_async::{
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
    sync_connection_wrapper::SyncConnectionWrapper,
};

use err_tools::{traceable::*, *};

async fn hello() -> &'static str {
    "hello fools"
}

async fn check_pass(Path((name, pass)): Path<(String, String)>) -> String {
    return format!("Hello to {} - {}", name, pass);
}

type CManager = SyncConnectionWrapper<SqliteConnection>;
type CPool = Pool<CManager>;

pub fn run_server() -> Result<(), TraceError> {
    println!("Running Server");

    let rt = tokio::runtime::Runtime::new().map_err(any_wrap!("Could not start runtime"))?;

    let db_url =
        dotenvy::var("DATABASE_URL").map_err(any_wrap!(".env DATABASE_URL not provided"))?;

    rt.block_on(async {
        //let connection = SyncConnectionWrapper::<SqliteConnection>::establish(&db_url).await.expect("Could not establish connection");

        let manager = AsyncDieselConnectionManager::<CManager>::new(&db_url);
        let pool: CPool = Pool::builder()
            .build(manager)
            .await
            .expect("Could not build connection pool");

        let app = Router::new()
            .route("/", get(hello))
            .route("/check/{name}/{pass}", get(check_pass))
            .with_state(pool);

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    Result::<(), TraceError>::Ok(())
}
