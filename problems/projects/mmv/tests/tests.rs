#![allow(unused)]

use clap::builder::Str;
use std::io;
use std::path::PathBuf;
use std::vec;

use mmv_lib::errors::*;
use mmv_lib::file_wrapper::*;
use mmv_lib::parser::*;

use mmv_lib::fill_in_output_pattern;
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

fn check_files(tmp: &TempDir, files: Vec<&str>) -> bool {
    for file in files {
        let file_path = tmp.path().join(file);
        if !file_path.exists() {
            return false;
        }
    }
    true
}

#[test]
fn test_fill_in_output_pattern() {
    assert_eq!(
        fill_in_output_pattern(
            "playground/a.txt",
            r"playground\/(.*)\.(.*)",
            "playground/#2.#1"
        )
        .ok(),
        Some(String::from("playground/txt.a"))
    );
    assert_eq!(
        fill_in_output_pattern(
            "playground/a.txt",
            r"playground\/(.*)\.(.*)",
            "playground/#1.#1"
        )
        .ok(),
        Some(String::from("playground/a.a"))
    );
    assert_eq!(
        fill_in_output_pattern(
            "playground/a.txt",
            r"playground\/(.*)\.(.*)",
            "playground/#2.#2"
        )
        .ok(),
        Some(String::from("playground/txt.txt"))
    );
    let err = fill_in_output_pattern(
        "playground/a.txt",
        r"playground\/(.*)\.(.*)",
        "playground/#3.#2",
    );
    assert!(err.is_err());
    let err = err.err().unwrap();
    match err {
        MassMoveError::TemplateMismatch(_, _) => {}
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_select_directory_name() {
    assert_eq!(
        select_directory_name(&PathBuf::from("a/b/c")).ok(),
        Some(PathBuf::from("a/b"))
    );
    assert_eq!(
        select_directory_name(&PathBuf::from("a")).ok(),
        Some(PathBuf::from(""))
    );
    let err = select_directory_name(&PathBuf::from("/"));
    assert!(err.is_err());
    let err = err.err().unwrap();
    match err {
        MassMoveError::TemplateWithoutFilename => {}
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_get_matched_filenames() {
    let files = vec!["a.bat", "b.bat", "a.txt", "b.txt", "aboba.txt", "boba"];
    let tmp_dir = TempDir::new("move_test").unwrap();
    create_files(&tmp_dir, files);
    let path = tmp_dir.path().to_path_buf();
    assert_eq!(
        get_matched_filenames(&path, &build_regex("a.*")).sort(),
        vec![PathBuf::from("a.bat"), PathBuf::from("a.txt")].sort()
    );
    assert_eq!(
        get_matched_filenames(&path, &build_regex("*a.*")).sort(),
        vec![
            PathBuf::from("a.bat"),
            PathBuf::from("a.txt"),
            PathBuf::from("aboba.txt")
        ]
        .sort()
    );
    assert_eq!(
        get_matched_filenames(&path, &build_regex("*a*")).sort(),
        vec![
            PathBuf::from("a.bat"),
            PathBuf::from("a.txt"),
            PathBuf::from("aboba.txt"),
            PathBuf::from("boba")
        ]
        .sort()
    );
}

#[test]
fn test_move_file_success() {
    let tests = vec![
        (
            vec!["aboba.txt"],
            ("aboba.txt", "boba.txt", false),
            vec!["boba.txt"],
        ),
        (
            vec!["aboba.txt", "boba.txt"],
            ("aboba.txt", "boba.txt", true),
            vec!["boba.txt"],
        ),
        (
            vec!["aboba.txt", "boba.txt"],
            ("aboba.txt", "boba.txt", false),
            vec!["boba.txt"],
        ),
    ];
    for (init, params, after) in tests {
        let tmp_dir = TempDir::new("move_test").unwrap();
        create_files(&tmp_dir, init);
        let old = tmp_dir.path().join(Path::new(params.0));
        let new = tmp_dir.path().join(Path::new(params.1));
        let force = params.2;
        move_file(old.as_path(), new.as_path(), force);
        assert!(check_files(&tmp_dir, after));
    }
}

#[test]
fn test_move_file_failed() {
    let tmp_dir = TempDir::new("move_test_fail").unwrap();
    create_files(&tmp_dir, vec!["aboba.txt", "boba.txt"]);
    let old = tmp_dir.path().join(Path::new("aboba.txt"));
    let new = tmp_dir.path().join(Path::new("boba.txt"));
    let err = move_file(old.as_path(), new.as_path(), false);
    assert!(err.is_err());
    let err = err.err().unwrap();
    match err {
        MassMoveError::FileAlreadyExists(_, _) => {}
        _ => {
            assert!(false)
        }
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
fn test_capture_regex_matches() {
    assert_eq!(
        capture_regex_matches(r"path\/to\/file\.(.*)", "path/to/file.dot").unwrap(),
        vec![String::from("dot")]
    );
    assert_eq!(
        capture_regex_matches(r"path\/to\/(.*)\.(.*)", "path/to/file.dot").unwrap(),
        vec![String::from("file"), String::from("dot")]
    );
    assert_eq!(
        capture_regex_matches(r"path\/to\/fi(.*)\.d(.*)", "path/to/file.dot").unwrap(),
        vec![String::from("le"), String::from("ot")]
    );
}

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
