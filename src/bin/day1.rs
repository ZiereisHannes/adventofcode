use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("inputs/input1.txt").expect("unable to open file");
    let input_reader = BufReader::new(file);

    let mut elves: Vec<u32> = Vec::new();

    for line in input_reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            elves.push(0);
            continue;
        }

        let calories: u32 = line.unwrap().parse().expect("can't parse");

        *elves.last_mut().unwrap() += calories;
    }

    println!("Part one: {}", elves.iter().max().unwrap());

    elves.sort();

    println!("Part two: {}", elves.iter().rev().take(3).sum::<u32>());
}
