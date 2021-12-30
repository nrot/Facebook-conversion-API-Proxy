use std::fmt::{Debug, Display};
use std::time;
use std::time::{Duration, SystemTime};

use log;
use tokio;

use chrono::{DateTime, Utc};

use rocket::serde;

use rocket::serde::json::Json;
use tokio::io::AsyncWrite;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct ElkConfig {
    host: String,
    port: u32,
    timeout: f64,
    index: String,
    password: String,
    runtime: Option<&'static tokio::runtime::Runtime>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElkEsc {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElkLogLevel {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ElkMessage {
    timestamp: String,
    index_name: String,
    password: String,
    tags: Vec<String>,
    esc: ElkEsc,
    message: Value,
    log: ElkLogLevel,
}

impl ElkConfig {
    pub fn new(
        host: String,
        port: u32,
        timeout: Option<f64>,
        index: String,
        password: String,
        runtime: &'static tokio::runtime::Runtime
    ) -> Self {
        ElkConfig {
            host: host,
            port: port,
            timeout: timeout.unwrap_or(0.0),
            index: index,
            password: password,
            runtime: Some(runtime)
        }
    }

    pub fn fake() -> Self {
        ElkConfig {
            host: "".into(),
            port: 0,
            timeout: 0.0,
            index: "".into(),
            password: "".into(),
            runtime: None
        }
    }

    pub fn send(&self, msg: Value, tags: Vec<String>, log_lvl: String) {
        let tmp: DateTime<Utc> = SystemTime::now().into();
        let dt = ElkMessage {
            timestamp: tmp.to_rfc3339(),
            index_name: self.index.clone(),
            password: self.password.clone(),
            tags: tags,
            esc: ElkEsc {
                version: "v1.0".into(),
            },
            log: ElkLogLevel { level: log_lvl },
            message: msg,
        };
        if self.host.is_empty() {
            log::info!("Fake logstash send: {:?}", dt);
            return;
        }
        let (host, port, tmo) = (self.host.clone(), self.port.clone(), self.timeout.clone());
        tokio::spawn(async move {
            match TcpStream::connect(format!("{}:{}", host, port)).await {
                Ok(mut connect) => {
                    log::debug!("Logstash connected");
                    match serde_json::to_string(&dt) {
                        Ok(s) => {
                            log::debug!("Prepare data send");
                            let _ =tokio::time::timeout(
                                time::Duration::from_secs_f64(tmo),
                                async {
                                    match connect.write_all(&s.as_bytes()).await{
                                        Ok(_)=>{log::debug!("Data send succes")},
                                        Err(e)=>{log::error!("Data can`t send: {:?}", e)}
                                    }
                                },
                            )
                            .await
                            .expect("Can`t write data to logstash");
                        }
                        Err(e) => {
                            log::error!("Can`t convert to json data: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Can`t connect to logstash: {:?}", e);
                }
            }
        });
    }

    pub fn send_info(&self, msg: Value) {
        self.send(
            msg,
            vec![String::from("log"), String::from("info")],
            "info".into(),
        );
    }
}
