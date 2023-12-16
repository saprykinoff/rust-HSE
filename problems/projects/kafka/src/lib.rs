

pub mod errors;


use std::{
    io::{Error, ErrorKind, Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use log::info;
use crate::errors::KafkaError;



fn read_json(stream: &TcpStream) -> Result<_, KafkaError> {
    unimplemented!();
}

fn user_register(stream: &TcpStream) -> bool {
    unimplemented!();
}

fn publisher_handler(stream: &TcpStream) {

}
fn subscriber_handler(stream: &TcpStream) {

}



pub fn run(ip: IpAddr, port: u16) {
    let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap();

}
