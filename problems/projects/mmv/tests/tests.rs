#![allow(unused)]

use clap::builder::Str;
use std::io;
use std::path::PathBuf;
use std::vec;

use mmv_lib::errors::*;
use mmv_lib::file_wrapper::*;
use mmv_lib::parser::*;

use std::fs::File;
use std::path::Path;
use tempdir::TempDir;

fn create_files(tmp: &TempDir, files: Vec<&str>) -> Result<(), io::Error> {
    for file in files {
        let file_path = tmp.path().join(file);
        File::create(file_path)?;
    }
    Ok(())
}

fn check_files(tmp: &TempDir, files: Vec<&str>) -> Result<bool, io::Error> {
    for file in files {
        let file_path = tmp.path().join(file);
        if !file_path.exists() {
            return Ok(false);
        }
    }
    Ok(true)
}

#[test]
fn test_fill_in_output_pattern() {
    let new_name = fill_in_output_pattern("playground/a.txt", r"playground\/(.*)\.(.*)", "playground/#2.#1");
    assert_eq!(new_name, "playground/txt.a");
}
#[test]
fn test_select_directory_name() {
    assert_eq!(select_directory_name(&PathBuf::from("a/b/c")), Ok(PathBuf::from("a/b")));
    assert_eq!(select_directory_name(&PathBuf::from("a")), Ok(PathBuf::from("")));
    assert_eq!(select_directory_name(&PathBuf::from("/")), Err(PathBuf::from("a/b")));
}
#[test]
fn test_get_matched_filenames() {}
#[test]
fn test_move_file() {
    let tests = vec![
        (
            vec!["aboba.txt"],
            ("aboba.txt", "boba.txt", false, None),
            vec!["boba.txt"],
        ),
        (
            vec!["aboba.txt", "boba.txt"],
            ("aboba.txt", "boba.txt", true, None),
            vec!["boba.txt"],
        ),
        (
            vec!["aboba.txt", "boba.txt"],
            (
                "aboba.txt",
                "boba.txt",
                false,
                Some(MassMoveError::FileAlreadyExists(
                    PathBuf::from("aboba.txt"),
                    PathBuf::from("boba.txt"),
                )),
            ),
            vec!["boba.txt"],
        ),
    ];
    for (init, params, after) in tests {
        let tmp_dir = TempDir::new("move_test").unwrap();
        create_files(&tmp_dir, init);
        let old = tmp_dir.path().join(Path::new(params.0));
        let new = tmp_dir.path().join(Path::new(params.1));
        let force = params.2;
        let err = params.3;
        let res = move_file(old.to_str().unwrap(), new.to_str().unwrap(), force);
        assert_eq!(res.err(), err);
        assert!(check_files(&tmp_dir, after).unwrap());
    }
}
#[test]
fn test_build_regex() {
    assert_eq!(build_regex("aboba*"), String::from("^aboba(.*)$"));
    assert_eq!(
        build_regex("path/to/file.*"),
        String::from(r"^path\/to\/file\.(.*)$")
    );
    assert_eq!(
        build_regex("and/even.[*]"),
        String::from(r"^and\/even\.\[(.*)\]$")
    );
}
#[test]
fn test_capture_regex_matches() {}
#[test]
fn test_parse_placeholders() {
    assert_eq!(
        parse_placeholders("aba#1"),
        (
            vec![1 as usize],
            vec![String::from("aba"), String::from("")],
        )
    );
    assert_eq!(
        parse_placeholders("aba##1##1#"),
        (
            vec![1, 1],
            vec![String::from("aba#"), String::from("#"), String::from("#")]
        )
    );
    assert_eq!(parse_placeholders("#"), (vec![], vec![String::from("#")],));
    assert_eq!(
        parse_placeholders("#ab#1ab#11ab"),
        (
            vec![1, 11],
            vec![String::from("#ab"), String::from("ab"), String::from("ab")],
        )
    );
}
