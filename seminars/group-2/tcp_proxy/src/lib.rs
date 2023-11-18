use std::{
    io::{self, BufReader, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    thread,
};

use log::{error, info};

fn proxy(mut reader: TcpStream, mut writer: TcpStream) {
    let mut buffer = [0; 1024];
    let reader_addr = reader.peer_addr().unwrap();
    let writer_addr = writer.peer_addr().unwrap();
    loop {
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => bytes_read,
            Err(err) if err.kind() == io::ErrorKind::Interrupted => {
                continue;
            }
            Err(err) => {
                error!("Failed while reading from {}: {:#?}", reader_addr, err);
                break;
            }
        };

        match writer.write_all(&buffer[..bytes_read]) {
            Ok(()) => info!("Written {} bytes to {}", bytes_read, writer_addr),
            Err(err) => {
                error!("Error while writing to {}: {:#?}", writer_addr, err)
            }
        }
    }

    info!("Close read from {}", reader_addr);
    reader.shutdown(Shutdown::Read);
    info!("Close write to {}", writer_addr);
    writer.shutdown(Shutdown::Write);
}

fn handle_connection(client_conn: TcpStream, dest: String) -> io::Result<()> {
    // client <---------------> server
    // client ---------> server
    // client <-------- server

    // client <---> proxy (we) <---> server
    //          ^                ^
    //     client_conn       server_conn

    let server_conn = match TcpStream::connect(dest) {
        Ok(conn) => conn,
        Err(_) => todo!(),
    };

    let client_addr = client_conn.peer_addr().unwrap();
    let server_addr = server_conn.peer_addr().unwrap();
    info!("Proxying connection {} <-> {}", client_addr, server_addr);

    let server_read = server_conn.try_clone().unwrap();
    let server_write = server_conn;
    let client_read = client_conn.try_clone().unwrap();
    let client_write = client_conn;

    thread::spawn(move || proxy(client_read, server_write));
    proxy(server_read, client_write);

    Ok(())
}

pub fn run_proxy(port: u16, dest: String) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        info!("New connection from {}", stream.peer_addr().unwrap());
        let dest = dest.clone();
        thread::spawn(move || handle_connection(stream, dest).unwrap());
    }
}
