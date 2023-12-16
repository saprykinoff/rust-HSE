#![allow(unused)]
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::result;
use kafka_lib::{read_json, ReceivedJSON};
use kafka_lib::errors::KafkaError;

fn read_json_wrapper(msg: &str) -> Result<ReceivedJSON, KafkaError> {
    let listener = TcpListener::bind("127.0.0.1:5343").unwrap();
    let mut sender = TcpStream::connect("127.0.0.1:5343").unwrap();
    sender.write(msg.as_bytes()).unwrap();
    let mut stream = listener.accept().unwrap().0;
    read_json(&mut stream)
}

#[test]
fn test_read_json_empty() {
    let result = read_json_wrapper(
        r#"{}"#
    ).unwrap();
    assert!(result.method.is_none());
    assert!(result.topic.is_none());
    assert!(result.message.is_none());
}


#[test]
fn test_read_json_corrupted() {
    let result = read_json_wrapper(
        r#"corrupted"#
    );
    assert!(result.is_err());
}

#[test]
fn test_read_json_publisher_register() {
    let result = read_json_wrapper(
        r#"{"method": "publish", "topic": "topic_name"}"#
    ).unwrap();
    assert_eq!(result.method, Some(String::from("publish")));
    assert_eq!(result.topic, Some(String::from("topic_name")));
    assert!(result.message.is_none());
}

#[test]
fn test_read_json_subscriber_register() {
    let result = read_json_wrapper(
        r#"{"method": "subscriber", "topic": "topic_name"}"#
    ).unwrap();
    assert_eq!(result.method, Some(String::from("subscriber")));
    assert_eq!(result.topic, Some(String::from("topic_name")));
    assert!(result.message.is_none());
}


#[test]
fn test_read_json_publisher_post() {
    let result = read_json_wrapper(
        r#"{"message": "Hello 1 from Rust publisher!"}"#
    ).unwrap();
    assert!(result.method.is_none());
    assert!(result.topic.is_none());
    assert_eq!(result.message, Some(String::from("Hello 1 from Rust publisher!")));
}

#[test]
fn test_read_json_all() {
    test_read_json_empty();
    test_read_json_corrupted();
    test_read_json_publisher_register();
    test_read_json_subscriber_register();
    test_read_json_publisher_post();
}

