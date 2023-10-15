#![allow(unused)]

pub mod parser;
pub mod errors;
pub mod file_wrapper;
pub mod regex_wrapper;

use parser::{parse_placeholders};
use crate::errors::MyError;
use crate::file_wrapper::{get_matched_filenames, move_file, split_dir_file};
use crate::parser::{build_regex, select_data};

pub fn insert_data(filename: &str, regex: &str, out : &str) -> Result<String, MyError> {
    let mut ans = String::new();
    let data = select_data(regex, filename);
    let (placeholders, strings) = parse_placeholders(out);

    ans.push_str(&strings[0]);
    for i in 1.. strings.len() {
        if placeholders[i - 1] > data.len() {
            return Err(MyError::UnexpextedAmountOfPlaceholders(data.len(), placeholders[i - 1]));
        }
        ans.push_str(& data[placeholders[i - 1] - 1]);
        ans.push_str(&strings[i]);
    }


    Ok(ans)
}

fn mass_move(template: &str, out: &str, force: bool) ->Result<(), MyError> {
    let (dir_name, file_name) = split_dir_file(template);
    let regex = build_regex(&file_name);
    let files = get_matched_filenames(&dir_name, &regex);
    for file in files {
        let filename = file.into_os_string().into_string().unwrap();
        let new_filename = insert_data(&filename, &regex, out)?;


        if !move_file(&filename, &new_filename, force) {
            Err(MyError::FileExists(filename, new_filename))
        }
    }

    Ok(())

}

