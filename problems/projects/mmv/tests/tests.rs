use mmv_lib::parser::*;
use mmv_lib::file_wrapper::*;
use mmv_lib::errors::*;



fn convert_vec<F, T> (vec: Vec<F>) -> Vec<T>
where T: From<F> {
    let mut ans = Vec::new();
    for x in vec {
        ans.push(x as T);
    }
    ans
}

#[test]
fn test_fill_in_output_pattern() {}
#[test]
fn test_mass_move() {}
#[test]
fn test_select_directory_name() {}
#[test]
fn test_get_matched_filenames() {}
#[test]
fn test_move_file() {}
#[test]
fn test_build_regex() {}
#[test]
fn test_capture_regex_matches() {}
#[test]
fn test_parse_placeholders() {
    let a = convert_vec::<i32, usize> (vec![1]);
    
    // let tests =
    // vec![
    //     ("aba#1", vec![1], vec!["aba", ""]),
    //     ("aba##1##1#", vec![1, 1], vec!["aba#", "#", "#"]),
    //     ("#", vec![], vec!["#"],),
    //     ("#", vec![], vec!["#"],)
    // ];
    // for (input, ans1, ans2) in tests {
    //     let res = parse_placeholders(input);
    //     assert_eq!(res.0 as Vec<i32>, ans1);
    //     assert_eq!(res.1 as Vec<Strin>, ans1);
    // }

    // assert_eq!(parse_placeholders("aba#1"), (convert_vec<i32, usize> (vec![1]), vec!["aba", ""],));
    // assert_eq!(
    //     parse_placeholders("aba##1##1#"),
    //     (vec![1, 1].into(), vec!["aba#", "#", "#"].into())
    // );
    // assert_eq!(parse_placeholders("#"), (vec![], vec!["#"],));
    // assert_eq!(parse_placeholders("#"), (vec![], vec!["#"],));
}
