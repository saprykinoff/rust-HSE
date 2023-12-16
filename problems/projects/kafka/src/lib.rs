#![allow(unused)]
pub mod errors;


use std::{
    io::{Error, ErrorKind, Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use log::info;
use serde::Deserialize;
use crate::errors::KafkaError;

#[derive(Deserialize, Debug)]
pub struct ReceivedJSON {
    method: Option<String>,
    topic: Option<String>,
    message: Option<String>,
}

pub fn read_json(stream: &mut TcpStream) -> Result<ReceivedJSON, KafkaError> {
    let mut json = String::new();
    stream.read_to_string(&mut json)?;
    let request: ReceivedJSON = serde_json::from_str(json.as_str())?;
    Ok(request)
}

pub fn user_register(stream: &TcpStream) -> bool {
    unimplemented!();
}

fn publisher_handler(stream: &TcpStream) {}

fn subscriber_handler(stream: &TcpStream) {}


pub fn run(ip: IpAddr, port: u16) {
    let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap();
}
