use clap::{App, Arg};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

fn subscribe_to_topic(stream: &mut TcpStream, topic_name: &str) -> io::Result<()> {
    let subscribe_command = format!(
        "{{\"method\": \"subscribe\", \"topic\": \"{}\"}}\n",
        topic_name
    );

    stream.write_all(subscribe_command.as_bytes())?;
    Ok(())
}

fn receive_messages(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            println!("Connection closed by server.");
            break;
        }

        let incoming_message =
            str::from_utf8(&buffer[..bytes_read]).expect("Failed to convert to UTF-8");

        println!("Received message: {}", incoming_message);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("Kafka Subscriber")
        .version("1.0")
        .about("Subscribes to a topic and receives messages")
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
    let topic_name = matches.value_of("topic").unwrap();
    let server_address = format!("{}:{}", address, port);

    let mut stream = TcpStream::connect(&server_address)?;

    println!("Connected to server at {}", server_address);

    subscribe_to_topic(&mut stream, topic_name)?;
    println!("Subscribed to topic '{}'", topic_name);

    receive_messages(stream)?;

    Ok(())
}
