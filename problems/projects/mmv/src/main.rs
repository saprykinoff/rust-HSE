use mmv_lib;
use mmv_lib::mass_move;
use clap::Parser;

/// Mass move utility analog on Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Pattern for files that should be moved. May contain * only in filename. Works only with utf-8 symbols.
    #[arg(short = 's', long = "src")]
    source: String,

    /// Pattern for new filenames. May contain #x, where 0 < x <= * count. Works only with utf-8 symbols.
    #[arg(short = 'd', long = "dst")]
    destination: String,

    /// Force mode overwrite files if they are already exist.
    #[arg(short = 'f', long = "force", default_value_t = false)]
    force_mode: bool,
}

fn main() {
    let args = Args::parse();
    let res = mass_move(&args.source, &args.destination, args.force_mode);
    if res.is_err() {
        println!("{:?}", res)
    }


}
