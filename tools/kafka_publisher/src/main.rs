use clap::{App, Arg};
use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn subscribe_to_topic(stream: &mut TcpStream, topic_name: &str) -> io::Result<()> {
    let subscribe_command = format!(
        "{{\"method\": \"subscribe\", \"topic\": \"{}\"}}\n",
        topic_name
    );

    stream.write_all(subscribe_command.as_bytes())?;
    Ok(())
}

fn publish_message(stream: &mut TcpStream, topic_name: &str, message: &str) -> io::Result<()> {
    let publish_command = format!(
        "{{\"method\": \"publish\", \"topic\": \"{}\", \"message\": \"{}\"}}\n",
        topic_name, message
    );

    stream.write_all(publish_command.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("Kafka Publisher")
        .version("1.0")
        .about("Publishes messages to a given topic")
        .arg(
            Arg::with_name("address")
                .short('a')
                .long("address")
                .value_name("ADDRESS")
                .help("Sets the server address to connect to")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Sets the server port to connect to")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("topic")
                .short('t')
                .long("topic")
                .value_name("TOPIC")
                .help("Sets the topic to subscribe to")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let address = matches.value_of("address").unwrap();
    let port = matches.value_of("port").unwrap();
    let server_address = format!("{}:{}", address, port);
    let topic_name = matches.value_of("topic").unwrap();

    let mut stream = TcpStream::connect(&server_address)?;

    println!("Connected to server at {}", server_address);

    subscribe_to_topic(&mut stream, topic_name)?;

    let mut i = 0;
    loop {
        let message = format!("Hello {} from Rust publisher!", i);
        i += 1;
        publish_message(&mut stream, topic_name, &message)?;
        println!("Published message: {}", message);
        thread::sleep(Duration::from_secs(1));
    }
}
