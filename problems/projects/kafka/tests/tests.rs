#![allow(unused)]

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::result;
use kafka_lib::{read_json, ReceivedJSON, user_register};
use kafka_lib::errors::KafkaError;

fn get_stream_with_message(msg: &str) -> TcpStream {
    let listener = TcpListener::bind("127.0.0.1:5343").unwrap();
    let mut sender = TcpStream::connect("127.0.0.1:5343").unwrap();
    sender.write(msg.as_bytes()).unwrap();
    let mut stream = listener.accept().unwrap().0;
    stream
}

#[test]
fn test_read_json_empty() {
    let mut stream = get_stream_with_message(
        r#"{}"#);
    let result = read_json(&mut stream).unwrap();
    assert!(result.method.is_none());
    assert!(result.topic.is_none());
    assert!(result.message.is_none());
}


#[test]
fn test_read_json_corrupted() {
    let mut stream = get_stream_with_message(
        r#"corrupted"#
    );
    let result = read_json(&mut stream);

    assert!(result.is_err());
}

#[test]
fn test_read_json_publisher_register() {
    let mut stream = get_stream_with_message(
        r#"{"method": "publish", "topic": "topic_name"}"#
    );
    let result = read_json(&mut stream).unwrap();
    assert_eq!(result.method, Some(String::from("publish")));
    assert_eq!(result.topic, Some(String::from("topic_name")));
    assert!(result.message.is_none());
}

#[test]
fn test_read_json_subscriber_register() {
    let mut stream = get_stream_with_message(
        r#"{"method": "subscriber", "topic": "topic_name"}"#
    );
    let result = read_json(&mut stream).unwrap();
    assert_eq!(result.method, Some(String::from("subscriber")));
    assert_eq!(result.topic, Some(String::from("topic_name")));
    assert!(result.message.is_none());
}


#[test]
fn test_read_json_publisher_post() {
    let mut stream = get_stream_with_message(
        r#"{"message": "Hello 1 from Rust publisher!"}"#
    );
    let result = read_json(&mut stream).unwrap();
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


#[test]
fn test_user_register() {
    let mut stream = get_stream_with_message(
        r#"{"method": "subscriber", "topic": "topic_name"}"#
    );
    let res = user_register(&mut stream);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), String::from("subscriber"));

    let mut stream = get_stream_with_message(
        r#"{"method": "publisher", "topic": "topic_name"}"#
    );
    let res = user_register(&mut stream);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), String::from("publisher"));

    let mut stream = get_stream_with_message(
        r#"{"mesage": "aboba"}"#
    );

    let res = user_register(&mut stream);
    assert!(res.is_err());

}

