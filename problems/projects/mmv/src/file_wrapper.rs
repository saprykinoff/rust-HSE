use std::fs;
use std::path::{Path, PathBuf};
use scan_dir::ScanDir;
use regex::Regex;

pub fn split_dir_file(template: &str ) -> (String, String){
    let res  = template.split_once('/');
    if res.is_none() {
        (String::from(""), String::from(template))
    } else {
        // let (a, b) =
        // (String::from(a), String::from(b))
        res.unwrap()
    }
}

pub fn get_matched_filenames(directory: &str, regex: &str) -> Vec<PathBuf> {
    let re = Regex::new(regex).unwrap();
    let files: Vec<_> = ScanDir::files().read(directory, |iter| {
        iter.filter(|(entry, _)| re.is_match(entry.path().file_name().unwrap().to_str().unwrap())).
            map(move |(entry, _)| entry.path())
            .collect()
    }).unwrap();

    files
}

pub fn move_file(from: &str, to: &str, force: bool) -> bool {
    let write = Path::new(to).exists() || force;
    if write {
        fs::rename(from, to);
    }
    write
}