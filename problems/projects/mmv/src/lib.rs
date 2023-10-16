#![allow(unused)]

pub mod errors;
pub mod file_wrapper;
pub mod parser;

use crate::errors::MassMoveError;
use crate::file_wrapper::{get_matched_filenames, move_file, split_directory_and_file_names};
use crate::parser::{build_regex, select_data};
use parser::parse_placeholders;

pub fn insert_data(filename: &str, regex: &str, out: &str) -> Result<String, MassMoveError> {
    let mut ans = String::new();
    let data = select_data(regex, filename);
    let (placeholders, strings) = parse_placeholders(out);

    ans.push_str(&strings[0]);
    for i in 1..strings.len() {
        if placeholders[i - 1] > data.len() {
            return Err(MassMoveError::TemplateMismatch(
                data.len(),
                placeholders[i - 1],
            ));
        }
        ans.push_str(&data[placeholders[i - 1] - 1]);
        ans.push_str(&strings[i]);
    }

    Ok(ans)
}

fn output(s: &str) {
    print!("{}", s);
}

pub fn mass_move(template: &str, out: &str, force: bool) -> Result<(), MassMoveError> {
    let (directory_name, file_name) = split_directory_and_file_names(template);
    let regex = build_regex(&template);
    let files = get_matched_filenames(&directory_name, &regex);
    if files.is_empty() {
        return Err(MassMoveError::NoFilesFound);
    }
    for file in files {
        let filename = file.into_os_string().into_string().unwrap();
        let new_filename = insert_data(&filename, &regex, out)?;
        output(&format!("Moving \"{filename}\" -> \"{new_filename}\" ..."));

        if !move_file(&filename, &new_filename, force) {
            output("Err\n");
            return Err(MassMoveError::FileAlreadyExists(filename.into(), new_filename.into()));
        }
        output("Ok\n");
    }

    Ok(())
}
