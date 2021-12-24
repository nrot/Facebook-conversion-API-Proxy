#[macro_use]
extern crate rocket;

use dotenv::dotenv;

use sqlx::sqlite::SqlitePoolOptions;
use std::fs::File;
use std::io::prelude::*;
use std::{env, str};

use rocket::State;
use sqlx::{Connection, Pool, Sqlite};

mod api;
mod auth;
mod elk;
mod handlers;

#[get("/")]
async fn index(_pool: &State<Pool<Sqlite>>) -> String {
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

    let logstash_host = env::var("LOGSTASH_HOST").unwrap_or("".into());
    let logstash_port = env::var("LOGSTASH_PORT")
        .unwrap_or("0".into())
        .parse::<u32>()
        .expect("LOGSTASH_PORT must be u32");
    let logstash_timeout: Option<u64> = match env::var("LOGSTASH_TIMEOUT") {
        Ok(v) => Some(v.parse::<u64>().expect("LOGSTASH_TIMEOUT must be u64")),
        Err(_) => None,
    };

    let logstash_index = env::var("LOGSTASH_INDEX").unwrap_or("".into());
    let logstash_password = env::var("LOGSTASH_PASSWORD").unwrap_or("".into());

    dbgs!("Try to connect sqlite DB by url: {}", &(db_url).clone());
    let sqlite_connection = match SqlitePoolOptions::new()
        .min_connections(sqlite_min_connection)
        .max_connections(sqlite_max_connection)
        .connect(db_url.as_str())
        .await
    {
        Ok(c) => c,
        Err(e) => panic!("Error by connect db: {}", e),
    };

    let mut create_sql_f =
        File::open("sql/create.sql").expect("Can`t open file with database struct");
    let mut buff = Vec::new();
    match create_sql_f.read_to_end(&mut buff) {
        Ok(_) => {}
        Err(e) => panic!("Error by reading file: {}", e),
    };
    let create_sql = match str::from_utf8(&buff) {
        Ok(s) => s,
        Err(e) => panic!("Error by encoding file as utf-8: {}", e),
    };

    let tmp_db = sqlite_connection.clone();
    sqlx::query(create_sql)
        .execute(&tmp_db)
        .await
        .expect("Can`t update DB struct");

    let _ = rocket::build()
        .manage(sqlite_connection)
        .manage(if !logstash_host.is_empty() && logstash_port != 0 {
            log::info!("Logstash init");
            elk::ElkConfig::new(
                logstash_host,
                logstash_port,
                logstash_timeout,
                logstash_index,
                logstash_password,
            )
        } else {
            log::info!("Logstash fake");
            elk::ElkConfig::fake()
        })
        .mount("/", routes![index, handlers::fb_proxy])
        .launch()
        .await;
}
