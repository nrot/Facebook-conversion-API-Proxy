use serde_json::{Value};
use rocket::serde::json::Json;
use rocket::State;
use crate::auth::ApiKey;
use log;
use crate::elk;


#[post("/fb-proxy", data="<data>", format="json")]
pub async fn fb_proxy(user: ApiKey, lg: &State<elk::ElkConfig>, data: Json<Value>)->String{
    log::debug!("fb-proxy data: {:?}", data);
    "Success".into()
}