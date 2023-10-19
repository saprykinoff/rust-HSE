pub mod errors;
pub mod file_wrapper;
pub mod parser;

use crate::errors::MassMoveError;
use crate::file_wrapper::{get_matched_filenames, move_file, select_directory_name};
use crate::parser::{build_regex, capture_regex_matches};
use parser::parse_placeholders;
use std::path::{Path, PathBuf};

/// Selects regex groups captures from [`filename`] using [`regex`]
/// Fills in this captures in [`output_template`] and returns the result
/// Returns [`MassMoveError::TemplateMismatch`] if number of placeholders exceed number of stars
///
/// #Example
///let new_name = fill_in_output_pattern("playground/a.txt", r"playground\/(.*)\.(.*)", "playground/#2.#1");
/// assert_eq!(new_name, "playground/txt.a")
///
///
pub fn fill_in_output_pattern(
    filename: &str,
    regex: &str,
    output_template: &str,
) -> Result<String, MassMoveError> {
    let mut ans = String::new();
    let data = capture_regex_matches(regex, filename)?;
    let (placeholders, strings) = parse_placeholders(output_template);

    ans.push_str(&strings[0]);
    for i in 1..strings.len() {
        if placeholders[i - 1] > data.len() {
            return Err(MassMoveError::TemplateMismatch(
                data.len(),
                placeholders[i - 1],
            ));
        }
        ans.push_str(&data[placeholders[i - 1] - 1]);
        ans.push_str(&strings[i]);
    }

    Ok(ans)
}

/// Moves all files that meet [`input_pattern`] into files according [`output_pattern`] with replacement of placeholders
/// If [`force_mode`] is true overwrites existing files
///
/// #Example
/// mass_move("playground/*.*", "playground/#2.#1") // Swap filename and extension for files in playground dir
///
pub fn mass_move(
    input_pattern: PathBuf,
    output_pattern: PathBuf,
    force_mode: bool,
) -> Result<(), MassMoveError> {
    let directory_name = select_directory_name(&input_pattern)?;
    let input_clone = input_pattern.clone();
    let output_clone = output_pattern.clone();
    let Some(input_pattern_str) = input_clone.to_str() else {
        return Err(MassMoveError::NonUTF8Symbol);
    };
    let Some(output_pattern_str) = output_clone.to_str() else {
        return Err(MassMoveError::NonUTF8Symbol);
    };
    let regex = build_regex(&input_pattern_str);
    let files = get_matched_filenames(&directory_name, &regex);
    if files.is_empty() {
        return Err(MassMoveError::NoFilesFound);
    }
    for file in files {
        let filename = file.into_os_string().into_string().unwrap();
        let new_filename = fill_in_output_pattern(&filename, &regex, output_pattern_str)?;
        print!("Moving \"{filename}\" -> \"{new_filename}\" ...");

        if let Err(error) = move_file(&Path::new(&filename), &Path::new(&new_filename), force_mode) {
            println!("Error");
            return Err(error);
        }
        print!("Ok\n");
    }
    Ok(())
}
