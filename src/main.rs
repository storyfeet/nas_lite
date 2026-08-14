mod models;
mod schema;
mod server;

use self::models::User;
use diesel::prelude::*;
use err_tools::{traceable::*, *};
use structopt::StructOpt;

use anyhow::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "nas_lite", about = "A NAS file server nothing more")]
enum Opt {
    AddUser(UserPassword),
    CheckPassword(UserPassword),
    Serve,
}

#[derive(Debug, StructOpt)]
#[structopt()]
struct UserPassword {
    name: String,
    password: String,
}

fn main() -> Result<(), TraceError> {
    let opt = Opt::from_args();

    dotenvy::dotenv().ok();

    match opt {
        Opt::AddUser(u) => return new_user(u),
        Opt::CheckPassword(u) => {
            _ = check_password(u)?;
        }
        Opt::Serve => return self::server::run_server(),
    }
    Result::<(), TraceError>::Ok(())
}

fn connect_database() -> Result<SqliteConnection, TraceError> {
    let db_url =
        dotenvy::var("DATABASE_URL").map_err(any_wrap!(".env DATABASE_URL not provided"))?;

    SqliteConnection::establish(&db_url).map_err(any_wrap!("Error connecting to DB : {}", db_url))
}

fn new_user(user_pass: UserPassword) -> Result<(), TraceError> {
    let mut connection = connect_database()?;

    let password_hash = bcrypt::hash(user_pass.password, bcrypt::DEFAULT_COST)
        .map_err(any_wrap!("Could not encrypt user password"))?;

    let user = crate::models::NewUser {
        user_name: user_pass.name,
        password_hash: password_hash,
    };

    diesel::insert_into(crate::schema::users::table)
        .values(&user)
        .execute(&mut connection)
        .map_err(any_wrap!("Could not insert user {}", &user.user_name))?;

    println!("User Inserted : {}", user.user_name);

    return Result::<(), TraceError>::Ok(());
}

fn check_password(user_pass: UserPassword) -> Result<bool, TraceError> {
    use self::schema::users::dsl::*;

    let mut connection = connect_database()?;

    let user_list = users
        .filter(user_name.eq(&user_pass.name))
        .limit(5)
        .select(User::as_select())
        .load(&mut connection)
        .map_err(any_wrap!("Could not load user by name {}", user_pass.name))?;

    for user in user_list {
        if bcrypt::verify(&user_pass.password, &user.password_hash)
            .map_err(any_wrap!("BCrypt couldn't verify password"))?
        {
            println!("User Password match: {}", &user.user_name);
            return t_ok(true);
        }
    }

    println!(
        "No User password match found for {} {}",
        user_pass.name, user_pass.password
    );
    t_ok(false)
}

fn t_ok<T>(res: T) -> Result<T, TraceError> {
    Result::<T, TraceError>::Ok(res)
}
