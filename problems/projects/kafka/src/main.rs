use clap::Parser;
use simplelog::*;

use std::net::IpAddr;

use kafka_lib;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short, long)]
    ip: IpAddr,

    #[clap(short, long, default_value = "0")]
    port: u16,
}

fn main() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
    .unwrap();

    let opts = Opts::parse();
    kafka_lib::run(opts.ip, opts.port);
}
