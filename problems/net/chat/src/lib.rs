#![forbid(unsafe_code)]

use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use log::info;

type Msg = Vec<u8>;

const END_MSG: u8 = 1u8;

pub fn read_msg(stream: &mut TcpStream) -> Result<Msg, Error> {
    let mut ans: Vec<u8> = Vec::new();
    let mut buf = [0u8; 1];
    while match stream.read(&mut buf) {
        Ok(x) => {
            ans.push(buf[0]);
            x == 1 && buf[0] != END_MSG
        }
        Err(err) => {
            return Err(err);
        }
    } {}
    if buf.last() != Some(&END_MSG) {
        return Err(Error::new(ErrorKind::Other, "oh no!"));
    }
    Ok(ans)
}

fn handle_user(mut stream: TcpStream, map: Arc<Mutex<HashMap<Msg, TcpStream>>>) {
    let Ok(my_name) = read_msg(&mut stream) else {
        return;
    };
    info!("Connect user: {:?}", my_name);
    let mut guard = map.lock().unwrap();
    guard.insert(my_name.clone(), stream.try_clone().unwrap());
    drop(guard);
    loop {
        let Ok(name) = read_msg(&mut stream) else {
            break;
        };
        info!("Read name {:?} from {:?}", name, my_name);
        let Ok(msg) = read_msg(&mut stream) else {
            break;
        };
        info!("Read msg {:?} from {:?}", msg, my_name);
        if my_name == name {
            info!("skipped");
            continue;
        }
        let mut guard = map.lock().unwrap();

        if let Some(x) = guard.get_mut(&name) {
            info!("Write name {:?} to {:?}", my_name, name);
            info!("Write message {:?} to {:?}", msg, name);
            let _ = x.write_all(my_name.as_slice());
            let _ = x.write_all(msg.as_slice());
        }

        info!("Sending from {:?} completed", my_name);
        drop(guard);
    }
    info!("Disconnect user: {:?}", my_name);
    let mut guard = map.lock().unwrap();
    guard.remove(&my_name);
    drop(guard);
}

pub fn run(ip: IpAddr, port: u16) {
    info!("Start");
    let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap();
    info!("Binded");
    let map = Arc::new(Mutex::new(HashMap::<Msg, TcpStream>::new()));
    for stream in listener.incoming() {
        info!("New connection");
        let Ok(stream) = stream else {
            continue;
        };
        let tmp = map.clone();
        thread::spawn(move || handle_user(stream, tmp));
    }
}
