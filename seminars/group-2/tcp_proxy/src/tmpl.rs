use clap::Parser;
use simplelog::*;

#[derive(Parser)]
struct Opts {
    #[clap(short, long, default_value = "0")]
    port: u16,

    #[clap(short, long)]
    dest: String,
}

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).unwrap();

    let opts = Opts::parse();
    run_proxy(opts.port, opts.dest);
}
