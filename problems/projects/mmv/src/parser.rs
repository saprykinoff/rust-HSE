#![allow(unused)]

use std::ops::Index;
use regex::Regex;
use crate::errors::*;


pub fn build_regex(template: &str) -> String {
    //builds correct regex from input template
    let mut ans = String::new();
    for ch in template.chars() {
        match ch {
            '*' => {
                ans.push('(');
                ans.push('.');
                ans.push(ch);
                ans.push(')');
            }
            //TODO replace to smth like      ch in "\\[]()..."
            '\\' | '[' | ']' | '(' | ')' | '^' | '$' | '.' | '|' | '?' | '+' | '/' => {
                ans.push('\\');
                ans.push(ch);
            }
            _ => {
                ans.push(ch);
            }
        }
    }
    ans
}


pub fn select_data(regex: &str, filename: &str) -> Vec<String> {
    //Capture data from filenames according template.
    //Return MyError::RegexError::NoMatch if filename doesn't match with template
    //Otherwise return Vec of this data
    let mut ans = Vec::new();
    let re = Regex::new(regex)?;

    let string_data = re.captures(filename).unwrap();
    println!("{:?}", string_data);
    println!("{:?}", string_data.len());
    for i in 1..string_data.len() {
        ans.push(string_data.index(i).to_string());
    }
    ans
}

pub fn parse_placeholders(out: &str) -> (Vec<usize>, Vec<String>) {

    //splits filename to Vec of placeholders and Vec of string parts

    let mut placeholders = Vec::new();
    let mut strings = Vec::new();
    strings.push(String::new());

    let mut placeholder = false;
    let mut cur:usize = 0;
    for ch in out.chars() {
        //TODO DONT WORKS WITH # IN FILENAMES
        if placeholder {
            if ch.is_ascii_digit() {
                cur = 10 * cur + (ch as u32 - '0' as u32);
                continue
            } else {
                if cur != 0 {
                    placeholders.push(cur);
                    cur = 0;
                    strings.push(String::new());
                } else {
                    strings.last_mut().unwrap().push('#');
                }
                placeholder = false;
            }
        }
        if ch == '#' {
            placeholder = true;
        } else {
            strings.last_mut().unwrap().push(ch)
        }
    }
    if placeholder {
        if cur != 0 {
            placeholders.push(cur);
            cur = 0;
            strings.push(String::new());
        } else {
            strings.last_mut().unwrap().push('#');
        }
        placeholder = false;
    }

    (placeholders, strings)
}