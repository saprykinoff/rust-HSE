#![allow(unused)]

use regex::Error;

#[derive(Debug)]
pub enum ParseError{
    RegexDontMatch,
    PlaceholderCount
}
#[derive(Debug)]
pub enum MyError {
    ParseError(ParseError),
    RegexError(regex::Error),
    FileExists(String, String),
    UnexpextedAmountOfPlaceholders(usize, usize)
}

impl From<regex::Error> for MyError {
    fn from(value: Error) -> Self {
        Self::RegexError(value)
    }
}