use std::fmt::Error;
use std::time::{Duration, SystemTime};
use std::slice::Iter;
use std::fmt;
use std::io::Cursor;
use std::fmt::{Debug, Display};

use log;

use chrono::{DateTime, Utc};

use tokio::{net::TcpStream, io::AsyncWriteExt};
use tokio::io::AsyncWrite;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug)]
pub struct ElkConfig{
    host: String,
    port: u32,
    timeout: u64,
    index: String,
    password: String,
    retry: Duration
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
    pub fn new(host: &dyn ToString, port: u32, timeout: Option<u64>, index: &dyn ToString, retry: Option<Duration>, password: String)->Self{
        ElkConfig{
            host: host.to_string(),
            port: port,
            timeout: timeout.unwrap_or(0),
            index: index.to_string(),
            retry: retry.unwrap_or(Duration::from_secs(5)),
            password: password
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
        match TcpStream::connect(format!("{}:{}", self.host, self.port)).await{
            Ok(mut connect)=>{
                match serde_json::to_string(&dt){
                    Ok(s)=>{
                        connect.write_all(&s.as_bytes());
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
        
    }
}