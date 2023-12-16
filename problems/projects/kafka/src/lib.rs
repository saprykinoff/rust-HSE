#![allow(unused)]

pub mod errors;

use std::{
    collections::HashMap,
    io::Read,
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};
use std::fmt::format;
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
                    json.clear(); //let's allow user to send any character until "{" has been received
                    continue;
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


type TopicList = Arc<Mutex<Vec<TcpStream>>>;

fn notify(stream: &mut TcpStream, s: &str) {
    let new_s = format!(r#"{{"error": {s}}}"#);
    stream.write(new_s.as_bytes());
    thread::sleep(Duration::from_millis(100)); // connection is closing before message being sent
}

fn validate_json(r: &Result<ReceivedJSON, KafkaError>, stream: &mut TcpStream) -> bool {
    if r.is_err() {
        if matches!(r.as_ref().unwrap_err(),KafkaError::DeserializationError(_)) {
            notify(stream, "received not valid json");
            info!("Failed to parse message from {}, closing connection", stream.peer_addr().unwrap());
        } else {
            info!("Error with {}: {:?}", stream.peer_addr().unwrap(), r.as_ref().unwrap_err())
        }
    }
    r.is_ok()
}

fn publisher_handler(mut stream: TcpStream,  topic_list: TopicList) {
    loop {
        let msg = read_json(&mut stream);
        if !validate_json(&msg, &mut stream) {
            break;
        }
        let msg = msg.unwrap();
        let Some(msg) = msg.message else {
            notify(&mut stream, "message should be sent");
            continue;
        };
        info!("Received from {}: \"{}\"", stream.peer_addr().unwrap(), msg);
        let mut guard = topic_list.lock().unwrap();
        for subscriber in guard.iter_mut() {
            let message = format!(r#"{{"message": {msg}}}"#) + "\n";
            subscriber.write(message.as_bytes());
            info!("{} -> {}: {}", stream.peer_addr().unwrap(), subscriber.peer_addr().unwrap(), message);
        }


    }
}


fn subscriber_handler(mut stream: TcpStream, ) {
    loop { //to keep connection alive
    }
}

pub fn run(ip: IpAddr, port: u16) {
    let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap();
    info!("Start kafka server on address {ip}:{port}");
    let mut topics_map: HashMap<String, TopicList> = HashMap::new();
    for stream in listener.incoming() {

        let Ok(mut stream) = stream else {
            error!("Declined connection: {:?}", stream.err().unwrap());
            continue;
        };

        let client_ip = stream.peer_addr().unwrap();
        debug!("New connection from {client_ip}");
        let reg = read_json(&mut stream);
        if (!validate_json(&reg, &mut stream)) {
            continue;
        }
        let reg = reg.unwrap();

        if (reg.method.is_none() || reg.topic.is_none()) {
            info!("To register {client_ip} should provide method and topic, closing connection.");
            continue;
        }
        let topic = reg.topic.unwrap();
        let method = reg.method.unwrap();
        if !topics_map.contains_key(&topic) {
            let v:Vec<TcpStream> = Vec::new();
            topics_map.insert(topic.clone(), Arc::new(Mutex::new(v)));
        }

        if method == String::from("publish") {
            info!("{client_ip} now is publisher in {}", topic);
            let topic_clone = Arc::clone(&topics_map[&topic]);
            thread::spawn(move || {
                publisher_handler(stream, topic_clone);
            });
        } else if method == String::from("subscribe") {
            info!("{client_ip} now is subscriber in {}", topic);
            let topic_clone = Arc::clone(&topics_map[&topic]);
            let mut guard = topic_clone.lock().unwrap();
            guard.push(stream.try_clone().unwrap());
            drop(guard);
            thread::spawn(move || {
                subscriber_handler(stream);
            });
        } else {
            info!("Unknown method {method}");
        }
    }
}

// {"method": "publish", "topic":"a"}
// {"method": "subscribe", "topic":"b"}
// {"message": "a"}