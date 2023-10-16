#![allow(unused)]

use std::ops::Index;
use regex::Regex;
use crate::errors::*;


fn escape_char(my_char: char) -> bool {
    let escape = r"\[]()^$.|?+/";
    for escaped_char in escape.chars() {
        if my_char == escaped_char {
            return true;
        }
    }
    false
}

pub fn build_regex(template: &str) -> String {
    //builds correct regex from input template
    let mut result = String::from('^');
    for char in template.chars() {
        match char {
            '*' => {
                result.push_str("(.*)");
            }
            char if escape_char(char) => {
                result.push('\\');
                result.push(char);
            }
            _ => {
                result.push(char);
            }
        }
    }
    result.push('$');
    result
}


pub fn select_data(regex: &str, filename: &str) -> Vec<String> {
    //Capture data from filenames according template.
    //Return MyError::RegexError::NoMatch if filename doesn't match with template
    //Otherwise return Vec of this data
    let mut result = Vec::new();
    let re = Regex::new(regex).unwrap();

    let captures = re.captures(filename);
    let string_data = captures.unwrap();
    for i in 1..string_data.len() {
        result.push(string_data.index(i).to_string());
    }
    result
}

pub fn parse_placeholders(out: &str) -> (Vec<usize>, Vec<String>) {

    //splits filename to Vec of placeholders and Vec of string parts

    let mut placeholders = Vec::new();
    let mut strings = Vec::new();
    strings.push(String::new());

    let mut placeholder = false;
    let mut current_num: usize = 0;
    for char in out.chars() {
        //TODO DONT WORKS WITH # IN FILENAMES
        if placeholder {
            if char.is_ascii_digit() {
                current_num = 10 * current_num + (char as usize - '0' as usize);
                continue;
            } else {
                if current_num != 0 {
                    placeholders.push(current_num);
                    current_num = 0;
                    strings.push(String::new());
                } else {
                    strings.last_mut().unwrap().push('#');
                }
                placeholder = false;
            }
        }
        if char == '#' {
            placeholder = true;
        } else {
            strings.last_mut().unwrap().push(char)
        }
    }
    if placeholder {
        if current_num != 0 {
            placeholders.push(current_num);
            current_num = 0;
            strings.push(String::new());
        } else {
            strings.last_mut().unwrap().push('#');
        }
        placeholder = false;
    }

    (placeholders, strings)
}