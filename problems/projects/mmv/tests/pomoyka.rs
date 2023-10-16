use mmv_lib::*;

fn test_select_dat() {
    let template = "path/to/some_*_filename.*";
    let filenames = vec![
        "path/to/some_A_filename.bin",
        "path/to/some_A_filename.jpg",
        "path/to/some_B_filename.bin",
        "path/to/some_B_filename.jpg",
    ];
    for filename in &filenames {
        let res = mmv_lib::parser::select_data(template, *filename);
        println!("{:?}", res);
    }
}

fn test_parse_placeholders() {
    let string = String::from("ab#1#ab##2#1");
    let res = mmv_lib::parser::parse_placeholders(string);
}
