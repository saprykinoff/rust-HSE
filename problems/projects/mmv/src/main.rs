#![allow(unused)]

use mmv_lib;
use mmv_lib::file_wrapper::get_matched_filenames;
use mmv_lib::mass_move;

fn main() {

    let res = mass_move("playground/txt*.*", "playground/txt.#2", true);
    if res.is_err() {
        println!("{:?}", res)
    }




}
