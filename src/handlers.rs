use serde_json::{Value};
use rocket::serde::json::Json;
use crate::auth::ApiKey;
use log;


#[post("/fb-proxy", data="<data>", format="json")]
pub async fn fb_proxy(user: ApiKey, data: Json<Value>)->String{
    log::debug!("fb-proxy data: {:?}", data);
    "Success".into()
}