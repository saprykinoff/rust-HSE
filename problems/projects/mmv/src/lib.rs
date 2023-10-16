#![allow(unused)]

pub mod parser;
pub mod errors;
pub mod file_wrapper;

use parser::{parse_placeholders};
use crate::errors::MyError;
use crate::file_wrapper::{get_matched_filenames, move_file, split_directory_and_file_names};
use crate::parser::{build_regex, select_data};

pub fn insert_data(filename: &str, regex: &str, out : &str) -> Result<String, MyError> {
    let mut ans = String::new();
    let data = select_data(regex, filename);
    let (placeholders, strings) = parse_placeholders(out);

    ans.push_str(&strings[0]);
    for i in 1.. strings.len() {
        if placeholders[i - 1] > data.len() {
            return Err(MyError::UnexpectedAmountOfPlaceholders(data.len(), placeholders[i - 1]));
        }
        ans.push_str(& data[placeholders[i - 1] - 1]);
        ans.push_str(&strings[i]);
    }


    Ok(ans)
}

fn output(s: &str) {
    print!("{}", s);
}

pub fn mass_move(template: &str, out: &str, force: bool) ->Result<(), MyError> {
    let (directory_name, file_name) = split_directory_and_file_names(template);
    let regex = build_regex(&template);
    let files = get_matched_filenames(&directory_name, &regex);
    if files.is_empty() {
        return Err(MyError::NoSuchFiles);
    }
    for file in files {
        let filename = file.into_os_string().into_string().unwrap();
        let new_filename = insert_data(&filename, &regex, out)?;
        output(&format!("Moving \"{filename}\" -> \"{new_filename}\" ..."));

        if !move_file(&filename, &new_filename, force) {
            output("Err\n");
            return Err(MyError::FileExists(filename, new_filename));
        }
        output("Ok\n");
    }

    Ok(())

}

