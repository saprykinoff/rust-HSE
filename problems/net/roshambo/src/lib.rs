#![forbid(unsafe_code)]

use std::{
    io::{Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    sync::mpsc::{channel, Receiver, RecvError, SendError, Sender},
    thread::{self, JoinHandle},
};

use log::{debug, warn};
#[derive(Debug)]
pub struct DualChannel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> DualChannel<T> {
    pub fn new() -> (DualChannel<T>, DualChannel<T>) {
        let (s1, r1) = channel();
        let (s2, r2) = channel();
        (
            DualChannel {
                sender: s1,
                receiver: r2,
            },
            DualChannel {
                sender: s2,
                receiver: r1,
            },
        )
    }
    pub fn get(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }

    pub fn send(&self, x: T) -> Result<(), SendError<T>> {
        self.sender.send(x)
    }
}

pub fn game_manager(first: DualChannel<u8>, second: DualChannel<u8>) {
    debug!("game have been started");
    loop {
        debug!("Round");
        let a = first.get().unwrap();
        debug!("first = {a} ");
        let b = second.get().unwrap();
        debug!("second = {b} ");
        if (a != b'R' && a != b'P' && a != b'S') || (b != b'R' && b != b'P' && b != b'S') {
            break;
        }
        let status_a;
        let status_b;
        if (a == b'R' && b == b'P') || (a == b'P' && b == b'S') || (a == b'S' && b == b'R') {
            status_a = b'L';
            status_b = b'W';
        } else if a == b {
            status_a = b'D';
            status_b = b'D';
        } else {
            status_b = b'L';
            status_a = b'W';
        }
        let _ = first.send(b);
        let _ = first.send(status_a);
        let _ = second.send(a);
        let _ = second.send(status_b);
    }
    debug!("game have been finished");
}

pub fn handle_player(mgr: DualChannel<u8>, mut stream: TcpStream) {
    debug!("Player connected");
    loop {
        let mut buf = [0u8; 1];
        if stream.read_exact(&mut buf).is_err() {
            break;
        }
        let _ = mgr.send(buf[0]);
        let mut buf = [0u8; 2];
        let res = mgr.get();
        if res.is_err() {
            break;
        }
        buf[0] = res.unwrap();

        let res = mgr.get();
        if res.is_err() {
            break;
        }
        buf[1] = res.unwrap();
        let Ok(r) = stream.write(&buf) else {
            break;
        };
        if r < 2 {
            break;
        };
    }
    debug!("Player disconnected");
}

pub fn run(ip: IpAddr, port: u16) {
    debug!("Start");
    let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap();
    debug!("Binded");
    let mut first: Option<DualChannel<u8>> = None;
    let second: DualChannel<u8>;

    let mut cnt = 0;
    let mut mgr_handler: Option<JoinHandle<()>> = None;
    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            warn!("Broken connection");
            break;
        };
        if cnt == 0 {
            debug!("first player..");
            let (tmp, first_tmp) = DualChannel::<u8>::new();
            first = Some(tmp);
            thread::spawn(move || {
                handle_player(first_tmp, stream);
            });
            cnt += 1;
            continue;
        }

        debug!("Second player..");
        let second_tmp: DualChannel<u8>;
        (second, second_tmp) = DualChannel::<u8>::new();
        thread::spawn(move || {
            handle_player(second_tmp, stream);
        });
        let tmp = first.unwrap();
        mgr_handler = Some(thread::spawn(move || {
            game_manager(tmp, second);
        }));
        break;
    }
    drop(listener);
    let _ = mgr_handler.unwrap().join();
}
