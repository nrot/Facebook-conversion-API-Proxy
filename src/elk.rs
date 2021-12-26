use std::fmt::Error;
use std::time::{Duration, SystemTime};
use std::slice::Iter;
use std::fmt;
use std::io::Cursor;
use std::fmt::{Debug, Display};

use log;

use chrono::{DateTime, Utc};

use rocket::serde;

use rocket::serde::json::Json;
use tokio::{net::TcpStream, io::AsyncWriteExt};
use tokio::io::AsyncWrite;

use rocket::serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug)]
pub struct ElkConfig{
    host: String,
    port: u32,
    timeout: u64,
    index: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElkEsc{
    pub version: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElkLogLevel{
    pub level: String
}


#[derive(Debug, Serialize, Deserialize)]
struct ElkMessage{
    timestamp: String,
    index_name: String,
    password: String,
    tags: Vec<String>,
    esc: ElkEsc,
    message: Value,
    log: ElkLogLevel
}

impl ElkConfig{
    pub fn new(host: String, port: u32, timeout: Option<u64>, index: String, password: String)->Self{
        ElkConfig{
            host: host,
            port: port,
            timeout: timeout.unwrap_or(0),
            index: index,
            password: password
        }
    }

    pub fn fake()->Self{
        ElkConfig{
            host: "".into(),
            port: 0,
            timeout: 0,
            index: "".into(),
            password: "".into()
        }
    }

    pub async fn send(&self, msg: Value, tags: Vec<String>, log_lvl: String)->Result<(), Error>{
        let tmp:DateTime<Utc> = SystemTime::now().into();
        let dt = ElkMessage{
            timestamp: tmp.to_rfc3339(),
            index_name: self.index.clone(),
            password: self.password.clone(),
            tags: tags,
            esc: ElkEsc{
                version: "v1.0".into()
            },
            log: ElkLogLevel{
                level: log_lvl
            },
            message: msg
        };
        if self.host.is_empty(){
            log::info!("Fake logstash send: {:?}", dt);
            return Ok(())
        }
        match TcpStream::connect(format!("{}:{}", self.host, self.port)).await{
            Ok(mut connect)=>{
                match serde_json::to_string(&dt){
                    Ok(s)=>{
                        connect.write_all(&s.as_bytes()).await;
                        Ok(())
                    },
                    Err(_)=>{
                        log::error!("Can`t convert to json data: {:?}", dt);
                        Ok(())
                    }
                }
            }
            Err(_)=>{
                Err(Error{})
            }
        }
    }

    pub async fn send_info(&self, msg: Value){
        self.send(msg, vec![String::from("log"), String::from("info")], "info".into());
    }
}