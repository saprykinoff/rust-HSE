#![allow(unused)]
pub mod errors;


use std::{
    io::{Error, ErrorKind, Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use log::info;
use log::error;
use serde::de::Unexpected::Str;
use serde::Deserialize;
use crate::errors::KafkaError;

#[derive(Deserialize, Debug)]
pub struct ReceivedJSON {
    pub method: Option<String>,
    pub topic: Option<String>,
    pub message: Option<String>,
}

pub fn read_json(stream: &mut TcpStream) -> Result<ReceivedJSON, KafkaError> {
    let mut json:Vec<u8> = Vec::new();
    let mut buf = [0u8; 1];
    let mut balance = 0;
    loop {
        match stream.read(&mut buf) {
            Ok(_) => {
                json.push(buf[0]);
                if (buf[0] == b'{') {
                    balance += 1;
                }
                if (balance == 0) {
                    KafkaError::JsonParseError(String::from("Message should start with \"{\""));
                }
                if (buf[0] == b'}') {
                    balance -= 1;
                }
                if (balance == 0) {
                    break;
                }
            }
            Err(_) => {}
        }
    }
    let req : Result<ReceivedJSON, _> = serde_json::from_slice(json.as_slice());
    match req  {
        Ok(x) => {
            Ok(x)
        }
        Err(err) => {
            Err(KafkaError::DeserializationError(err))
        }
    }
}

pub fn user_register(stream: &mut TcpStream) -> Result<bool, KafkaError> {
    let reg = read_json(stream)?;
    Ok(true)
}

fn publisher_handler(stream: &mut TcpStream) {}

fn subscriber_handler(stream: &TcpStream) {}


pub fn run(ip: IpAddr, port: u16) {
    let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap();
}
