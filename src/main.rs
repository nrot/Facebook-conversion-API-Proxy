#[macro_use]
extern crate rocket;

use dotenv::dotenv;

use sqlx::sqlite::SqlitePoolOptions;
use std::{env, thread};

use rocket::State;
use sqlx::{sqlite, Connection, Pool, Sqlite, SqlitePool};

mod api;
mod auth;
mod database;

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


    rocket::build()
        .manage(sqlite_connection)
        .mount("/", routes![index])
        .launch()
        .await;
}
