use crate::auth::ApiKey;
use crate::elk;
use log;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::Value;

#[post("/fb-proxy", data = "<data>", format = "json")]
pub async fn fb_proxy(user: ApiKey, lg: &State<elk::ElkConfig>, data: Json<Value>) -> String {
    log::debug!("fb-proxy data: {:?}", data);
    let sender = Box::new(lg.inner().clone());
    tokio::spawn(async move { sender.send_info(Value::from(data.to_string())).await });
    "Success".into()
}
