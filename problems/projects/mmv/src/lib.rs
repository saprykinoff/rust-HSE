#![allow(unused)]

pub mod errors;
pub mod file_wrapper;
pub mod parser;

use crate::errors::MassMoveError;
use crate::file_wrapper::{get_matched_filenames, move_file, split_directory_and_file_names};
use crate::parser::{build_regex, select_data};
use parser::parse_placeholders;

pub fn fill_in_output_pattern(filename: &str, regex: &str, out: &str) -> Result<String, MassMoveError> {
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


pub fn mass_move(input_pattern: &str, output_pattern: &str, force_mode: bool) -> Result<(), MassMoveError> {
    let (directory_name, file_name) = split_directory_and_file_names(input_pattern);
    let regex = build_regex(&input_pattern);
    let files = get_matched_filenames(&directory_name, &regex);
    if files.is_empty() {
        return Err(MassMoveError::NoFilesFound);
    }
    for file in files {
        let filename = file.into_os_string().into_string().unwrap();
        let new_filename = fill_in_output_pattern(&filename, &regex, output_pattern)?;
        print!("Moving \"{filename}\" -> \"{new_filename}\" ...");

        if let Err(error) = move_file(&filename, &new_filename, force_mode) {
            println!("Error");
            return Err(error);
        }
        print!("Ok\n");
    }
    Ok(())
}
