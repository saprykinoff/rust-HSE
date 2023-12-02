use clap::{App, Arg};
use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn register_as_publisher(stream: &mut TcpStream, topic_name: &str) -> io::Result<()> {
    let register_command = format!(
        "{{\"method\": \"publish\", \"topic\": \"{}\"}}\n",
        topic_name
    );

    stream.write_all(register_command.as_bytes())?;
    Ok(())
}

fn publish_message(stream: &mut TcpStream, message: &str) -> io::Result<()> {
    let publish_command = format!("{{\"message\": \"{}\"}}\n", message);

    stream
        .write_all(publish_command.as_bytes())
        .expect("Not able to publish message");

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
                .help("Sets the topic to publish to")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let address = matches.value_of("address").unwrap();
    let port = matches.value_of("port").unwrap();
    let server_address = format!("{}:{}", address, port);
    let topic_name = matches.value_of("topic").unwrap();

    let mut stream = TcpStream::connect(&server_address)
        .expect(format!("Not able to connect to {:?}", server_address).as_str());

    register_as_publisher(&mut stream, topic_name)
        .expect(format!("Not able to register as publisher to {:?}", topic_name).as_str());

    let mut i = 0;
    loop {
        let message = format!("Hello {} from Rust publisher!", i);
        i += 1;
        publish_message(&mut stream, &message)?;
        println!("Published message: {}", message);
        thread::sleep(Duration::from_secs(1));
    }
}
