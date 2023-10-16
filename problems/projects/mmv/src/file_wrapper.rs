use regex::Regex;
use scan_dir::ScanDir;
use std::fs;
use std::path::{Path, PathBuf};

pub fn split_directory_and_file_names(template: &str) -> (String, String) {
    let res = template.split_once('/');
    if res.is_none() {
        (String::from(""), String::from(template))
    } else {
        let (a, b) = res.unwrap();
        (String::from(a), String::from(b))
    }
}

pub fn get_matched_filenames(directory: &str, regex: &str) -> Vec<PathBuf> {
    let re = Regex::new(regex).unwrap();
    let files = ScanDir::files().read(directory, |iter| {
        iter.filter(move |(entry, _)| re.is_match(entry.path().into_os_string().to_str().unwrap()))
            .map(|(entry, _)| entry.path())
            .collect()
    });
    if files.is_err() {
        Vec::new()
    } else {
        files.unwrap()
    }
}

pub fn move_file(from: &str, to: &str, force: bool) -> bool {
    if from == to {
        return true;
    }
    let write = (!Path::new(to).exists()) || force;
    if write {
        fs::rename(from, to);
    }
    write
}
