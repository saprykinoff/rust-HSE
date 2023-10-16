#![allow(unused)]

use std::path::PathBuf;


#[derive(Debug)]
pub enum MassMoveError {
    RegexError(regex::Error),
    StdIoError(std::io::Error),
    FileAlreadyExists(PathBuf, PathBuf),
    TemplateMismatch(usize, usize),
    NoFilesFound,
}

impl From<regex::Error> for MassMoveError {
    fn from(value: regex::Error) -> Self {
        Self::RegexError(value)
    }
}

impl From<std::io::Error> for MassMoveError {
    fn from(value: std::io::Error) -> Self {
        Self::StdIoError(value)
    }
}
