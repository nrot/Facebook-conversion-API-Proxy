use std::pin::Pin;
use std::task::{Context};

use rocket::State;
use sqlx::{Database, Pool, Sqlite};

use rocket::request::{FromRequest, Outcome, Request};
use rocket::fairing::Fairing;


#[derive(Debug)]
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
        Outcome::Success(ApiKey{
            User: "Name".into(),
            Token: "Token".into()
        })
    }
}
