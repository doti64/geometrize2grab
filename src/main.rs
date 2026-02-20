use std::io::*;
use std::fs;

use crate::conversion::parse_geometrize_data;

mod builder;
mod conversion;

fn main() {
    println!("geometrize2GRAB!!! [ v1.0.0 ] ");
    println!("input path to geometrize json data, to make one go to the following link");
    println!("https://www.samcodes.co.uk/project/geometrize-haxe-web/");

    let mut path = String::new();

    stdin()
    .read_line(&mut path)
    .expect("failed to read line somehow");

    let path = path.trim();

    println!("[i] generating level base");
    crate::builder::generate_level_base();
    println!("[i] converting json data to str");

    let data = fs::read_to_string(path)
        .expect("failed to read file :(");

    println!("[+] sucessfully parsed json data hopefully");
    println!("[*] converting... (can take a min)");
    parse_geometrize_data(&data);
    println!("level generated!");
}
