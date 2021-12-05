use std::pin::Pin;
use std::task::{Context};

use rocket::State;
use sqlx::{Database, FromRow, Pool, Sqlite};

use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;
use log;


#[derive(Debug, sqlx::FromRow)]
pub struct ApiKey{
    User: String,
    Token: String
}

#[derive(Debug)]
pub enum ApiKeyError{
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey{
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>)-> Outcome<Self, Self::Error>{
        //let db = req.guard::<Pool<Sqlite>>().await;
        let db = match req.rocket().state::<Pool<Sqlite>>() {
            Some(d)=> d,
            None => panic!("DB Pool<Sqlite> must be set as managed")
        };
        let token =  req.headers().get_one("Authorization").unwrap_or("");
        match sqlx::query_as::<_, ApiKey>("SELECT * FROM Token WHERE Token = ?").bind(token).fetch_one(db).await {
            Ok(k)=>{
                log::debug!("Result: {:?}", k);
                Outcome::Success(ApiKey{
                    User: "Name".into(),
                    Token: "Token".into()
                })
            },
            Err(e)=>{
                log::debug!("Failure: {:?}", e);
                Outcome::Failure((Status::Unauthorized ,ApiKeyError::Invalid))
            }
        }
    }
}
