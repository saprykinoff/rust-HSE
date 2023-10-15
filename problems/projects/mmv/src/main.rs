#![allow(unused)]

use mmv_lib;
use mmv_lib::file_wrapper::get_matched_filenames;

fn main() {

    let res = get_matched_filenames("src", r"^*a*$");
    println!("{:?}", res);




}
