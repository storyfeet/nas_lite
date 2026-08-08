use structopt::StructOpt;
use err_tools::{*,traceable::*};
use diesel::prelude::*;

use anyhow::*;


#[derive(Debug, StructOpt)]
#[structopt(name="nas_lite", about="A NAS file server nothing more")]
enum Opt{
    AddUser(NewUser),
    Serve,
}

#[derive(Debug, StructOpt)]
#[structopt()]
struct NewUser{
    name:String,
    password:String,
}

fn main() ->Result<(),TraceError> {
    let opt = Opt::from_args();

    dotenvy::dotenv().ok();

    match opt{
        Opt::AddUser(u) => {return new_user(u)}
        Opt::Serve => {println!("Serving")}
    }
    Result::<(),TraceError>::Ok(())

}

fn connect_database()->Result<SqliteConnection,TraceError> {
    let db_url = dotenvy::var("DATABASE_URL")
        .map_err(any_wrap!(".env DATABASE_URL not provided"))?;
        

    SqliteConnection::establish(&db_url)
        .map_err(any_wrap!("Error connecting to DB : {}",db_url))

}


fn new_user(new_user:NewUser) ->Result<(),TraceError>{
    let _ = connect_database()?;

    println!("I got a connection for {} {}", new_user.name, new_user.password);
    
    return Result::<(),TraceError>::Ok(())
}
