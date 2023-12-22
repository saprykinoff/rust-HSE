#![allow(unused)]

pub mod errors;

use crate::errors::KafkaError;
use log::{debug, error, info};
use serde::Deserialize;
use std::fmt::format;
use std::io::Write;
use std::time::Duration;
use std::{
    collections::HashMap,
    io::Read,
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

#[derive(Deserialize, Debug)]
pub struct ReceivedJSON {
    pub method: Option<String>,
    pub topic: Option<String>,
    pub message: Option<String>,
}


/// Function that read `ReceivedJSON` from [`stream`].
///
/// # Arguments
/// `stream`- strem to read
///
/// # Examples
/// ```
/// use std::net::{TcpStream, TcpListener};
/// use kafka_lib::{read_json, ReceivedJSON};
/// use std::io::Write;
/// let listener = TcpListener::bind("127.0.0.1:5343").unwrap();
/// let mut sender = TcpStream::connect("127.0.0.1:5343").unwrap();
/// sender.write_all(r#"{"message" : "aboba"}"#.as_bytes());
/// let mut stream = listener.accept().unwrap().0;
/// let recv = read_json(&mut stream).unwrap();
/// assert_eq!(recv.message, Some(String::from("aboba")));
/// assert!(recv.method.is_none());
/// assert!(recv.topic.is_none());
/// ```
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
            Err(err) => {
                return Err(KafkaError::IoError(err));
            }
        }
    }
    let req: Result<ReceivedJSON, _> = serde_json::from_slice(json.as_slice());
    match req {
        Ok(x) => Ok(x),
        Err(err) => Err(KafkaError::DeserializationError(err)),
    }
}

type TopicList = Arc<Mutex<Vec<TcpStream>>>;


/// Function to notify user about an error
fn notify(stream: &mut TcpStream, s: &str) {
    let new_s = format!(r#"{{"error": {s}}}"#);
    stream.write(new_s.as_bytes());
    thread::sleep(Duration::from_millis(100)); // connection is closing before message being sent
}

/// Function to handle result and notify user if there is user-side error

fn validate_json(r: &Result<ReceivedJSON, KafkaError>, stream: &mut TcpStream) -> bool {
    if r.is_err() {
        if matches!(r.as_ref().unwrap_err(), KafkaError::DeserializationError(_)) {
            notify(stream, "received not valid json");
            info!(
                "Failed to parse message from {}, closing connection",
                stream.peer_addr().unwrap()
            );
        } else {
            info!(
                "Error with {}: {:?}",
                stream.peer_addr().unwrap(),
                r.as_ref().unwrap_err()
            )
        }
    }
    r.is_ok()
}

fn publisher_handler(mut stream: TcpStream, topic_list: TopicList) {
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
        let message = format!(r#"{{"message": "{msg}"}}"#) + "\n";
        for subscriber in guard.iter_mut() {
            subscriber.write(message.as_bytes());
            info!(
                "{} -> {}: {}",
                stream.peer_addr().unwrap(),
                subscriber.peer_addr().unwrap(),
                message
            );
        }
    }
}

fn keep_alive(mut stream: TcpStream) {
    loop { //to keep connection alive
    }
}

/// Kafka provides three main functions to its users:
/// Publish and subscribe to streams of records
/// Effectively store streams of records in the order in which records were generated
/// Process streams of records in real time

pub fn run(ip: IpAddr, port: u16) {
    let Ok(listener) = TcpListener::bind(format!("{ip}:{port}")) else {
        info!("Port is busy");
        return;
    };
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
            let v: Vec<TcpStream> = Vec::new();
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
                keep_alive(stream);
            });
        } else {
            info!("Unknown method {method}");
        }
    }
}
