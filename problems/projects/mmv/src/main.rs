use std::path::PathBuf;
use clap::Parser;
use mmv_lib;
use mmv_lib::errors::MassMoveError;
use mmv_lib::mass_move;

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
    let res = mass_move(PathBuf::from(args.source), PathBuf::from(args.destination), args.force_mode);
    let Some(err) = res.err() else  {
        return;
    };
    match err {
        MassMoveError::RegexError(err) => {println!("{:?}", err)}
        MassMoveError::StdIoError(err) => {println!("{:?}", err)}
        MassMoveError::FileAlreadyExists(old, new) => {println!("Can not overwrite {old} -> {new}. Use -f for this")}
        MassMoveError::TemplateMismatch(limit, found) => {println!("Wrong output template: found #{found} while max is #{limit}")}
        MassMoveError::NoFilesFound => {println!("No files found for source template")}
        MassMoveError::CaptureRegexError => {println!("Can not capture matches from template")}
        MassMoveError::TemplateWithoutFilename => {println!("Source should define files, not folders")}
        MassMoveError::NonUTF8Symbol => {println!("Only UTF-8 symbols is supported")}
    }

}
