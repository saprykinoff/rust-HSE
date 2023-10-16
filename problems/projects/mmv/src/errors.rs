#![allow(unused)]

use std::path::PathBuf;
use regex::Error;

#[derive(Debug)]
pub enum MassMoveError {
    RegexError(regex::Error),
    FileAlreadyExists(PathBuf, PathBuf),
    TemplateMismatch(usize, usize),
    NoFilesFound,
}

impl From<regex::Error> for MassMoveError {
    fn from(value: Error) -> Self {
        Self::RegexError(value)
    }
}
