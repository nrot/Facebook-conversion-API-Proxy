#[macro_use]
extern crate rocket;

use dotenv::dotenv;

use sqlx::sqlite::SqlitePoolOptions;
use std::{env, thread, str};
use std::io::prelude::*;
use std::fs::File;

use rocket::State;
use sqlx::{sqlite, Connection, Pool, Sqlite, SqlitePool};

mod api;
mod auth;
mod handlers;

#[get("/")]
async fn index(pool: &State<Pool<Sqlite>>) -> String {
    String::from("Hello world!")
}

#[rocket::main]
async fn main() {
    dbgs!("Debug server");

    println!("Server start");

    dotenv().ok();

    let db_url = env::var("SQLITE_URL").expect("SQLITE_URL must be set in env");

    let sqlite_max_connection = env::var("SQLITE_MAX_CONNECTION")
        .unwrap_or(String::from("16"))
        .parse::<u32>()
        .expect("SQLITE_MAX_CONNECTION must be u32");

    let sqlite_min_connection = env::var("SQLITE_MIN_CONNECTION")
        .unwrap_or(String::from("1"))
        .parse::<u32>()
        .expect("SQLITE_MIN_CONNECTION must be u32");

    dbgs!(
        "Try to connect sqlite DB by url: {}",
        db_url.as_str().clone()
    );
    let sqlite_connection = match SqlitePoolOptions::new()
        .min_connections(sqlite_min_connection)
        .max_connections(sqlite_max_connection)
        .connect(db_url.as_str())
        .await
    {
        Ok(c) => c,
        Err(e) => panic!("Error by connect db: {}", e),
    };

    let mut create_sql_f = File::open("sql/create.sql").expect("Can`t open file with database struct");
    let mut buff = Vec::new();
    match create_sql_f.read_to_end(&mut buff) {
        Ok(_)=>{},
        Err(e)=>panic!("Error by reading file: {}", e)
    };
    let create_sql = match str::from_utf8(&buff){
        Ok(s)=>s,
        Err(e) => panic!("Error by encoding file as utf-8: {}", e)
    };

    let tmp_db = sqlite_connection.clone();
    sqlx::query(create_sql).execute(&tmp_db).await.expect("Can`t update DB struct");

    let _ = rocket::build()
        .manage(sqlite_connection)
        .mount("/", routes![index, handlers::fb_proxy])
        .launch()
        .await;
}
