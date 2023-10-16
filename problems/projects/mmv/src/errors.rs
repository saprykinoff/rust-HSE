#![allow(unused)]

use regex::Error;


#[derive(Debug)]
pub enum MyError {
    RegexError(regex::Error),
    FileExists(String, String),
    UnexpectedAmountOfPlaceholders(usize, usize),
    NoSuchFiles,
}

impl From<regex::Error> for MyError {
    fn from(value: Error) -> Self {
        Self::RegexError(value)
    }
}