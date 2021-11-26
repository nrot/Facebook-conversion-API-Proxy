#[macro_use] extern crate rocket;

use dotenv::dotenv;
use env_logger;

use log::LevelFilter;
use sqlx::sqlite::SqlitePoolOptions;
use std::{env, thread};

use sqlx::{sqlite, Connection, SqlitePool};

mod api;
mod auth;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("")
}

#[launch]
async fn main() -> std::io::Result<()> {
    dbgs!("Debug server");

    println!("Server start");

    dotenv().ok();

    let cpus = env::var("CPU_WORKERS")
        .expect("CPU_WORKERS must be set in env")
        .parse::<usize>()
        .expect("CPU_WORKERS must be usize");

    let log_level = match env::var("LOG_LEVEL") {
        Ok(lvl) => match lvl.to_uppercase().trim() {
            "OFF" => LevelFilter::Off,
            "ERROR" => LevelFilter::Error,
            "WARN" => LevelFilter::Warn,
            "INFO" => LevelFilter::Info,
            "DEBUG" => LevelFilter::Debug,
            "TRACE" => LevelFilter::Trace,
            "" => LevelFilter::Info,
            _ => panic!("LOG_LEVEL must be OFF, ERROR, WARN, INFO, DEBUG, TRACE"),
        },
        Err(_) => LevelFilter::Info,
    };

    let _ = env_logger::builder()
        .parse_default_env()
        .filter_level(if cfg!(debug_assertions) {
            LevelFilter::Debug
        } else {
            log_level
        })
        .try_init();

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
    let sqlite_connection = match block_on(
        SqlitePoolOptions::new()
            .min_connections(sqlite_min_connection)
            .max_connections(sqlite_max_connection)
            .connect(db_url.as_str())
    ) {
        Ok(c) => c,
        Err(e) => panic!("Error by connect db: {}", e),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(sqlite_connection.clone())
            .wrap(auth::TokenAuthInit::new(sqlite_connection.clone()))
        // .service(hello)
        // .service(echo)
        // .route("/hey", web::get().to(manual_hello))
    })
    .workers(if cfg!(debug_assertions) { 1 } else { cpus })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
