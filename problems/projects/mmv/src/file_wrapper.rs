use crate::errors::MassMoveError;
use regex::Regex;
use scan_dir::ScanDir;
use std::fs;
use std::path::{Path, PathBuf};

///Splits directory name and file name from path.
///Returns pair (directory name, file name)
pub fn split_directory_and_file_names(path: &str) -> (String, String) {
    let res = path.split_once('/');
    if res.is_none() {
        (String::from(""), String::from(path))
    } else {
        let (a, b) = res.unwrap();
        (String::from(a), String::from(b))
    }
}

///Scans directory and returns names of files in directory that satisfy regex pattern
pub fn get_matched_filenames(directory: &str, regex: &str) -> Vec<PathBuf> {
    let re = Regex::new(regex).unwrap();
    let files = ScanDir::files().read(directory, |iter| {
        iter.filter(move |(entry, _)| {
            // println!("{}", entry.path().into_os_string().to_str().unwrap());
            re.is_match(entry.path().into_os_string().to_str().unwrap())
        })
        .map(|(entry, _)| entry.path())
        .collect()
    });
    if files.is_err() {
        Vec::new()
    } else {
        files.unwrap()
    }
}

///Renames file from old_name to new_name.
///Returns [`MassMoveError::FileAlreadyExists`] if a path [`new_name`]  is already present in the file system and [`force_mode`] is false
pub fn move_file(old_name: &str, new_name: &str, force_mode: bool) -> Result<(), MassMoveError> {
    //TODO replace to PathBuf

    if old_name == new_name {
        return Ok(());
    }
    let write = (!Path::new(new_name).exists()) || force_mode;
    if write {
        fs::rename(old_name, new_name)?;
        Ok(())
    } else {
        Err(MassMoveError::FileAlreadyExists(
            PathBuf::from(old_name),
            PathBuf::from(new_name),
        ))
    }
}
