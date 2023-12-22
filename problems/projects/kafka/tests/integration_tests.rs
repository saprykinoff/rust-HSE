#![allow(unused)]

use rand::Rng;
use std::io::{self, prelude::*};
use std::mem::take;
use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream};
use std::process::{Child, Command};
use std::str::{self, FromStr};
use std::thread;
use std::time::{self, Duration};

const BINARY_PATH: &str = env!("CARGO_BIN_EXE_kafka");

const SOH: u8 = 1;

fn take_a_nap() {
    thread::sleep(Duration::from_millis(100));
}

enum IpVersion {
    V4,
    V6,
}

struct ServerWrapper {
    proc: Option<Child>,
    addr: SocketAddr,
}

impl ServerWrapper {
    fn start(ip_version: IpVersion) -> Self {
        let mut rng = rand::thread_rng();
        let port = rng.gen_range(40000..49151);
        let ip = match ip_version {
            IpVersion::V4 => IpAddr::from_str("127.0.0.1").unwrap(),
            IpVersion::V6 => IpAddr::from_str("::1").unwrap(),
        };

        eprintln!("binary path {}", BINARY_PATH);
        let proc = Command::new(BINARY_PATH)
            .arg("--ip")
            .arg(ip.to_string())
            .arg("--port")
            .arg(port.to_string())
            .spawn()
            .unwrap();
        thread::sleep(time::Duration::from_millis(1000));
        Self {
            proc: Some(proc),
            addr: SocketAddr::new(ip, port),
        }
    }

    fn is_alive(&mut self) -> bool {
        self.proc
            .as_mut()
            .map_or(false, |proc| proc.try_wait().unwrap().is_none())
    }

    fn stop(&mut self) -> std::io::Result<()> {
        self.proc.take().map_or(Ok(()), |mut proc| proc.kill())
    }
}

impl Drop for ServerWrapper {
    fn drop(&mut self) {
        let _ = self.stop().unwrap();
    }
}

#[derive(Debug)]
struct Client {
    topic: String,
    conn: TcpStream,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            topic: self.topic.clone(),
            conn: self.conn.try_clone().unwrap(),
        }
    }
}

impl Client {
    // fn start(server_addr: SocketAddr) -> std::io::Result<Self> {
    //     let conn = TcpStream::connect(server_addr)?;
    //     Ok(Self { topic, conn })
    // }
    // fn register(&mut self, as_who: &str, topic: &str) -> std::io::Result<()>  {
    //     self.conn.write_all(format!("{{\"method\": \"{as_who}\", \"topic\": \"{topic}\"}}").as_bytes())
    // }
    fn start_publisher(server_addr: SocketAddr, topic: &str) -> std::io::Result<Self> {
        let mut conn = TcpStream::connect(server_addr)?;
        conn.write_all(format!("{{\"method\": \"publish\", \"topic\": \"{topic}\"}}").as_bytes());
        Ok(Self {
            topic: String::from(topic),
            conn,
        })
    }
    fn start_subscriber(server_addr: SocketAddr, topic: &str) -> std::io::Result<Self> {
        let mut conn = TcpStream::connect(server_addr)?;
        conn.write_all(format!("{{\"method\": \"subscribe\", \"topic\": \"{topic}\"}}").as_bytes());
        Ok(Self {
            topic: String::from(topic),
            conn,
        })
    }

    fn publish(&mut self, message: &str) -> std::io::Result<()> {
        self.conn
            .write_all(format!("{{\"message\": \"{message} (topic: {})\"}}", self.topic).as_bytes())
    }

    fn expect_message(&mut self, expected: &str) {
        let expected = format!("{{\"message\": \"{expected} (topic: {})\"}}\n", self.topic);
        let mut buf = vec![0; expected.len()];
        self.conn.read_exact(&mut buf).unwrap();
        let actual = String::from_utf8(buf).unwrap();
        assert_eq!(expected, actual);
    }

    fn expect_nothing(&mut self) -> io::Result<()> {
        self.conn.set_nonblocking(true).unwrap();
        let mut buf = [0; 1];
        let is_empty = match self.conn.peek(&mut buf) {
            Ok(n) => n == 0,
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => true,
            Err(err) => {
                self.conn.set_nonblocking(false).unwrap();
                return Err(err);
            }
        };
        self.conn.set_nonblocking(false).unwrap();
        assert!(is_empty);
        Ok(())
    }

    fn shutdown(&mut self, how: Shutdown) {
        let _ = self.conn.shutdown(how);
    }
}

#[test]
fn test_simple_hello() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut pub1 = Client::start_publisher(server.addr, "a").unwrap();
    let mut sub1 = Client::start_subscriber(server.addr, "a").unwrap();
    pub1.publish("Hello!");
    sub1.expect_message("Hello!");
}

#[test]
fn test_publishers_subscriber() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut pub1 = Client::start_publisher(server.addr, "a").unwrap();
    let mut pub2 = Client::start_publisher(server.addr, "a").unwrap();
    let mut pub3 = Client::start_publisher(server.addr, "a").unwrap();

    let mut sub1 = Client::start_subscriber(server.addr, "a").unwrap();
    take_a_nap();

    pub1.publish("Hello from 1!");
    take_a_nap();
    pub2.publish("Hello from 2!");
    take_a_nap();
    pub3.publish("Hello from 3!");
    sub1.expect_message("Hello from 1!");
    sub1.expect_message("Hello from 2!");
    sub1.expect_message("Hello from 3!");
}

#[test]
fn test_publishers_subscribers() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut pub1 = Client::start_publisher(server.addr, "a").unwrap();
    let mut pub2 = Client::start_publisher(server.addr, "a").unwrap();
    let mut pub3 = Client::start_publisher(server.addr, "a").unwrap();

    let mut sub1 = Client::start_subscriber(server.addr, "a").unwrap();
    let mut sub2 = Client::start_subscriber(server.addr, "a").unwrap();
    let mut sub3 = Client::start_subscriber(server.addr, "a").unwrap();
    take_a_nap();
    pub1.publish("Hello from 1!");
    take_a_nap();
    pub2.publish("Hello from 2!");
    take_a_nap();
    pub3.publish("Hello from 3!");
    sub1.expect_message("Hello from 1!");
    sub1.expect_message("Hello from 2!");
    sub1.expect_message("Hello from 3!");
    sub2.expect_message("Hello from 1!");
    sub2.expect_message("Hello from 2!");
    sub2.expect_message("Hello from 3!");
    sub3.expect_message("Hello from 1!");
    sub3.expect_message("Hello from 2!");
    sub3.expect_message("Hello from 3!");
}

#[test]
fn test_publishers_subscribers_multitopic() {
    let server = ServerWrapper::start(IpVersion::V4);

    let addr = server.addr;
    let topics = vec!["a", "b", "CCDsd"];
    for topic in topics {
        thread::spawn(move || {
            let mut pub1 = Client::start_publisher(server.addr, topic).unwrap();
            let mut pub2 = Client::start_publisher(server.addr, topic).unwrap();
            let mut pub3 = Client::start_publisher(server.addr, topic).unwrap();

            let mut sub1 = Client::start_publisher(server.addr, topic).unwrap();
            let mut sub2 = Client::start_publisher(server.addr, topic).unwrap();
            let mut sub3 = Client::start_publisher(server.addr, topic).unwrap();
            take_a_nap();

            pub1.publish("Hello from 1!");
            take_a_nap();
            pub2.publish("Hello from 2!");
            take_a_nap();
            pub3.publish("Hello from 3!");
            sub1.expect_message("Hello from 1!");
            sub1.expect_message("Hello from 2!");
            sub1.expect_message("Hello from 3!");
            sub2.expect_message("Hello from 1!");
            sub2.expect_message("Hello from 2!");
            sub2.expect_message("Hello from 3!");
            sub3.expect_message("Hello from 1!");
            sub3.expect_message("Hello from 2!");
            sub3.expect_message("Hello from 3!");
        });
    }
}
