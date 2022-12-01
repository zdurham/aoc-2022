use std::env;
use std::fs;

fn read_file() {
    let file_path = "./src/day1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");


    println!("With text:\n {contents}")
}

pub fn day1() {
    read_file()
}
