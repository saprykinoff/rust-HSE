#![allow(unused)]
pub mod errors;

use std::{
    collections::HashMap,
    io::Read,
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};
use std::io::Write;
use std::time::Duration;
use crate::errors::KafkaError;
use log::{error, debug, info};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct ReceivedJSON {
    pub method: Option<String>,
    pub topic: Option<String>,
    pub message: Option<String>,
}

pub fn read_json(stream: &mut TcpStream) -> Result<ReceivedJSON, KafkaError> {
    let mut json: Vec<u8> = Vec::new();
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
                    return Err(KafkaError::JsonParseError(String::from("Message should start with \"{\"")));
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
    let req: Result<ReceivedJSON, _> = serde_json::from_slice(json.as_slice());
    match req {
        Ok(x) => Ok(x),
        Err(err) => Err(KafkaError::DeserializationError(err)),
    }
}


type TopicList = Mutex<Vec<TcpStream>>;
type TopicsMap = Mutex<HashMap<String, Arc<TopicList>>>;


fn publisher_handler(mut stream:  TcpStream, topic: String) {
    loop {

    }
}

fn subscriber_handler(mut stream: TcpStream, topic: String) {
    loop {

    }
}

pub fn run(ip: IpAddr, port: u16) {

    let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap();
    info!("Start kafka server on address {ip}:{port}");
    for stream in listener.incoming() {
        debug!("New connection");
        let Ok(mut stream) = stream else {
            error!("Declined connection: {:?}", stream.err().unwrap());
            continue
        };

        let client_ip = stream.peer_addr().unwrap();
        debug!("Connected {client_ip}");
        let reg = read_json(&mut stream);
        debug!("Received: {:?}", reg);
        let Ok(reg) = reg else {
            if matches!(reg.as_ref().unwrap_err(),KafkaError::JsonParseError(_)) ||
                matches!(reg.as_ref().unwrap_err(),KafkaError::DeserializationError(_))  {
                stream.write(r#"{"error": "received not valid json"}"#.as_bytes());
                thread::sleep(Duration::from_millis(100)); // connection is closing before message being sent
                info!("Failed to parse message from {client_ip}, closing connection")
            }
            continue;
        };
        if (reg.method.is_none() || reg.topic.is_none()) {
            info!("To register {client_ip} should provide method and topic");
        }
        let topic = reg.topic.unwrap();
        let method = reg.method.unwrap();

        if method == String::from("publisher") {
            debug!("{client_ip} now is publisher in {}", topic);
            thread::spawn(move || {
                publisher_handler(stream, topic);
            });
        } else if method == String::from("subscriber")  {
            debug!("{client_ip} now is subscriber in {}", topic);
            thread::spawn(move || {
                subscriber_handler(stream, topic);
            });
        }
    }
}
