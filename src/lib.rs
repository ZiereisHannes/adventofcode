use std::{fs::File, io::BufReader};

pub fn open_input() -> BufReader<std::fs::File> {
    let file = File::open("inputs/input3.txt").expect("error opening file");
    BufReader::new(file)
}

pub fn open_example() -> BufReader<std::fs::File> {
    let file = File::open("inputs/example3.txt").expect("error example");
    BufReader::new(file)
}

// fn fetch_input() -> 
