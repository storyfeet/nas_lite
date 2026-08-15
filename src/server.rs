use anyhow::*;
use axum::{
    Router,
    extract::{Path, State},
    routing::{get, post},
};
use diesel::{prelude::*, sqlite::SqliteConnection};
use diesel_async::{
    RunQueryDsl,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
    sync_connection_wrapper::SyncConnectionWrapper,
};

use crate::errors::NasRes;

use err_tools::{traceable::*, *};

async fn hello() -> &'static str {
    "hello fools"
}

async fn check_pass(
    Path((name, pass)): Path<(String, String)>,
    State(cpool): State<CPool>,
) -> String {
    //Result<String, TraceError> {
    use crate::models::User;
    use crate::schema::users::dsl::*;

    let mut con = cpool.get().await.unwrap();
    //.map_err(any_wrap!("Could not access connection pool"))?;

    let user_list = users
        .filter(user_name.eq(&name))
        .limit(5)
        .select(User::as_select())
        .load(&mut con)
        .await
        .unwrap(); //.map_err(any_wrap!("Could not run load user by name {}", &name))?;

    let mut found = false;
    for user in user_list {
        if bcrypt::verify(&pass, &user.password_hash).unwrap()
        //.map_err(any_wrap!("BCrypt couldn't verify password"))?
        {
            found = true;
        }
    }

    if found {
        return format!("Password found for {}", &name);
    }
    return format!("Password not found for {} ", &name);
    //return Result::<String, TraceError>::Ok(format!("Hello to {} - {}", name, pass));
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
            .route("/login", post(hello))
            .with_state(pool);

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("localhost:3000")
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    Result::<(), TraceError>::Ok(())
}

async fn login(Path((name, pass)): Path<(String, String)>) -> NasRes<String> {
    return NasRes::ok("Hello".to_string());
}
