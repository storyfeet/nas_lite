pub mod db_util;
mod file_manager;
mod session;
use crate::common_types as CT;
use crate::models as MD;
use crate::schema as SCH;
use anyhow::*;
use axum::{
    Router,
    extract::{Json, Path, State},
    response,
    routing::{get, post},
};
use db_util as DB;
use diesel::prelude::*;
use diesel_async::{
    RunQueryDsl,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};
use file_manager::FileManager;
use std::path::PathBuf;

use crate::errors::{ResponseResult, res_err, res_ok};

use err_tools::{traceable::*, *};

async fn hello() -> &'static str {
    "hello fools"
}

#[derive(Clone)]
struct SState {
    pool: DB::CPool,
    f_man: FileManager,
}

async fn check_pass(
    Path((name, pass)): Path<(String, String)>,
    State(state): State<SState>,
) -> String {
    //Result<String, TraceError> {
    use crate::models::User;
    use crate::schema::users::dsl::*;

    let mut con = state.pool.get().await.unwrap();
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

pub fn run_server() -> Result<(), TraceError> {
    println!("Running Server");

    let rt = tokio::runtime::Runtime::new().map_err(any_wrap!("Could not start runtime"))?;

    let db_url =
        dotenvy::var("DATABASE_URL").map_err(any_wrap!(".env DATABASE_URL not provided"))?;

    rt.block_on(async {
        //let connection = SyncConnectionWrapper::<SqliteConnection>::establish(&db_url).await.expect("Could not establish connection");

        let manager = AsyncDieselConnectionManager::<DB::CManager>::new(&db_url);
        let pool: DB::CPool = Pool::builder()
            .build(manager)
            .await
            .expect("Could not build connection pool");

        let f_man = FileManager::new("ungit/server_side").await;

        let state = SState { pool, f_man };

        let app = Router::new()
            .route("/", get(hello))
            .route("do_thing", post(do_thing))
            .route("/check/{name}/{pass}", get(check_pass))
            .route("/upload_file", post(upload_file))
            .route("/login", post(login))
            .with_state(state);

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("localhost:3000")
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    Result::<(), TraceError>::Ok(())
}

async fn login(
    State(mut state): State<SState>,
    Json(user_pass): Json<CT::UserPassword>,
) -> ResponseResult<response::Json<CT::SessionData>> {
    let check = session::check_user_pass(user_pass, &mut state.pool)
        .await? //Result
        .ok_or::<TraceError>(err_at!("User Password not found"))?;

    let sess = session::create_user_session(check, &mut state.pool).await?;

    res_ok(response::Json(sess))
}

async fn do_thing(State(state): State<SState>) -> String {
    "do_thing".to_string()
}

async fn upload_file(
    State(mut state): State<SState>,
    Json(file_upload): Json<CT::FileUpload>,
) -> ResponseResult<String> {
    let sess_data = session::check_session(file_upload.token, &mut state.pool)
        .await?
        .ok_or::<err_tools::traceable::TraceError>(err_at!("User does not exist"))?;

    let mut con = state
        .pool
        .get()
        .await
        .map_err(any_wrap!("Could not access connection pool"))?;

    use SCH::files::columns as FCOL;

    let file_rec = SCH::files::dsl::files
        .filter(FCOL::file_hash.eq(&file_upload.file_hash))
        .filter(FCOL::user_id.eq(sess_data.user_id))
        .filter(FCOL::file_type.eq(CT::FileType::File))
        .select(MD::File::as_select())
        .first(&mut con)
        .await
        .optional()
        .map_err(any_wrap!("Could not get file by key",))?;

    let (is_new_file, mut f_data): (bool, MD::File) = match file_rec {
        Some(f) => (false, f),
        None => (
            true,
            MD::File {
                file_hash: file_upload.file_hash.clone(),
                file_type: CT::FileType::File,
                file_name: file_upload.file_name,
                file_size: file_upload.file_size as i64,
                chunk_size: file_upload.chunk_size as i64,
                chunks_loaded: 0,
                completed: None,
                created: chrono::Utc::now().naive_utc(),
                modified: chrono::Utc::now().naive_utc(),
                deleted: None,
            },
        ),
    };

    if file_upload.chunk_size != f_data.chunk_size as u64
        || file_upload.chunk_num != f_data.chunks_loaded as u64
        || file_upload.file_size != f_data.file_size as u64
    {
        return res_err(err_at!("Sent chunk does not match needed next chunk"));
    }

    let f_path: PathBuf = [
        &(sess_data.user_id.to_string()),
        "file",
        &file_upload.file_hash,
    ]
    .iter()
    .collect();

    let offset = file_upload.chunk_size * file_upload.chunk_num;

    let data_len = file_upload.data.0.len() as u64;

    if data_len != file_upload.chunk_size && offset + data_len != file_upload.file_size {
        return res_err(err_at!("sent data chunk incorrect size"));
    }

    let written = state
        .f_man
        .write_chunk(file_upload.data.0, f_path.clone(), offset)
        .await
        .map_err(|e| {
            let an: anyhow::Error = e.into();
            err_wrap!("Could not write to file: ")(an)
        })?;

    if written as u64 != data_len {
        return res_err(err_at!("Could not write whole chunk to file"));
    }

    f_data.chunks_loaded += 1;

    if f_data.chunks_loaded as i64 * f_data.chunk_size as i64 >= f_data.file_size {
        // check hash matches, if so mark complete
        let hash = state.f_man.read_hash(f_path).await.map_err(|e| {
            let an: anyhow::Error = e.into();
            err_wrap!("Could not read file for hash")(an)
        })?;

        if hash != file_upload.file_hash {
            // TODO work out how to reset file upload - probably set chunk to zero and delete
            return res_err(err_at!("Hash did not match"));
        }
        f_data.completed = Some(chrono::Utc::now().naive_utc());
    }

    use SCH::files::dsl as FSL;
    let _ = diesel::insert_into(SCH::files::table)
        .values(&f_data)
        .on_conflict((FSL::file_hash, FSL::user_id, FSL::file_type))
        .do_update()
        .set(&f_data)
        .execute(&mut con)
        .await
        .map_err(|e| {
            let an: anyhow::Error = e.into();
            err_wrap!("Could not read file for hash")(an)
        })?;

    res_ok("Trace".to_string())
}
