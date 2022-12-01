use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("./input.txt").expect("error opening file");
    let buf = BufReader::new(file);

    let mut elves = Vec::new();

    for line in buf.lines() {
        let line = line.expect("unable to read line");

        if line.is_empty() {
            elves.push(0);
            continue;
        }

        let cal: u32 = line.parse().unwrap();
        *elves.last_mut().unwrap() += cal; 
    }

    println!("Part one: {}", elves.iter().max().unwrap());

    elves.sort();
    let sum: u32 = elves.iter().rev().take(3).sum();

    println!("Part two: {}", sum);
}
