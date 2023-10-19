use crate::errors::*;
use regex::Regex;
use std::ops::Index;

/// Checks if [`checked_char`] should be escaped in regex
///
/// #Example
/// assert!(!escape_char('a'))
/// assert!(escape_char('['))
///
fn escape_char(checked_char: char) -> bool {
    let escape = r"\[]()^$.|?+/";
    for escaped_char in escape.chars() {
        if checked_char == escaped_char {
            return true;
        }
    }
    false
}

/// Builds regex that meet the [`file_pattern`] .
/// Uses regex groups to capture data.
///
/// #Example
/// let regex = build_regex("path/to/file*.*")
/// assert_eq!(regex, r"^path\/to\/file(.*)\.(.*)$")
///
pub fn build_regex(file_pattern: &str) -> String {
    let mut result = String::from('^');
    for char in file_pattern.chars() {
        match char {
            '*' => {
                result.push_str("(.*)");
            }
            char if escape_char(char) => {
                result.push('\\');
                result.push(char);
            }
            _ => {
                result.push(char);
            }
        }
    }
    result.push('$');
    result
}

/// Captures pattern matches from [`filename`].
/// Returns [`MassMoveError::CaptureRegexError`] if filename doesn't match with the [`regex`] template
/// Otherwise returns Vec of matched groups
///
/// #Example
/// let captures = capture_regex_matches(r"(.*)_(.*)", "ab_c").unwrap();
/// assert_eq!("ab", "c");
///
///
pub fn capture_regex_matches(regex: &str, filename: &str) -> Result<Vec<String>, MassMoveError> {
    let mut result = Vec::new();
    let re = Regex::new(regex)?;

    let captures = re.captures(filename);
    let string_data = captures.ok_or(MassMoveError::CaptureRegexError)?;
    for i in 1..string_data.len() {
        result.push(string_data.index(i).to_string());
    }
    Ok(result)
}

/// Devides [`output_pattern`] by placeholders.
/// Returns numbers of placeholder and splitted strings.
///
/// #Example
/// let (placeholders, strings) = parse_placeholders("a#2, b#10")
/// assert_eq!(placeholders, vec! ("a", ", b", ""))
/// assert_eq!(strings , vec! (2, 10))
///
pub fn parse_placeholders(output_template: &str) -> (Vec<usize>, Vec<String>) {
    let mut placeholders = Vec::new();
    let mut strings = Vec::new();
    strings.push(String::new());

    let mut placeholder = false;
    let mut current_num: usize = 0;
    for char in output_template.chars() {
        if placeholder {
            if char.is_ascii_digit() {
                current_num = 10 * current_num + (char as usize - '0' as usize);
                continue;
            } else {
                if current_num != 0 {
                    placeholders.push(current_num);
                    current_num = 0;
                    strings.push(String::new());
                } else {
                    strings.last_mut().unwrap().push('#');
                }
                placeholder = false;
            }
        }
        if char == '#' {
            placeholder = true;
        } else {
            strings.last_mut().unwrap().push(char)
        }
    }
    if placeholder {
        if current_num != 0 {
            placeholders.push(current_num);
            strings.push(String::new());
        } else {
            strings.last_mut().unwrap().push('#');
        }
    }

    (placeholders, strings)
}
