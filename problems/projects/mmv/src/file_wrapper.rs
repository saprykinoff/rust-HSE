use crate::errors::MassMoveError;
use regex::Regex;
use scan_dir::ScanDir;
use std::fs;
use std::path::{Path, PathBuf};

/// Splits directory name and file name from path.
/// Returns pair (directory name, file name)
///
/// #Example
/// assert_eq!(split_directory_and_file_names("path/to/file"), ("path/to", "file"))
///
pub fn select_directory_name(path: &PathBuf) -> Result<PathBuf, MassMoveError> {
    let Some(res) = path.parent() else {
        return Err(MassMoveError::TemplateWithoutFilename);
    };
    Ok(res.to_path_buf())
}

///Scans directory and returns names of files in directory that satisfy regex pattern
///
/// #Example
/// assert_eq!(get_matched_filenames(".", "*.txt"), vec!(PathBuf::from("a.txt"), PathBuf::from("b.txt"))
/// assert_eq!(get_matched_filenames(".", "a.*"), vec!(PathBuf::from("a.txt"), PathBuf::from("a.bat"))
///
pub fn get_matched_filenames(directory: &PathBuf, regex: &str) -> Vec<PathBuf> {
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

/// Renames file from old_name to new_name.
/// Returns [`MassMoveError::FileAlreadyExists`] if a path [`new_name`]  is already present in the file system and [`force_mode`] is false
///
/// #Example
///
/// assert!(move_file("a.txt", "a.bat", false).is_err());
/// assert!(move_file("a.txt", "a.bat", true).is_ok());
///
///
pub fn move_file(old_name: &Path, new_name: &Path, force_mode: bool) -> Result<(), MassMoveError> {

    if old_name == new_name {
        return Ok(());
    }
    let write = (!new_name.exists()) || force_mode;
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
