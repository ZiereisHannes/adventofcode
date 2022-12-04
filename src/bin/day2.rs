use std::{fs::File, io::{BufReader, BufRead}};
use adventofcode::get_input; 

fn main() {
    let example = "A Y
B X
C Z";

    // let file = File::open("inputs/input2.txt").expect("error opening file");
    // let buf = BufReader::new(file);
    let buf = get_input();

    let mut score_one: u32 = 0;
    let mut score_two: u32 = 0;

    for line in buf.lines() {
        let play = line.unwrap();

        score_one += part_one(play.as_str());
        score_two += part_two(play.as_str());
    }

    println!("Part one: {}", score_one); 
    println!("Part two: {}", score_two);
}

fn part_one(play: &str) -> u32 {

    match play {
        "A Y" => 8,
        "B X" => 1,
        "C Z" => 6,
        "B Y" => 5,
        "C X" => 7,
        "B Z" => 9,
        "C Y" => 2,
        "A Z" => 3,
        "A X" => 4,
        _ => panic!("{}", play),
    }
}

fn part_two(play: &str) -> u32 {
    match play {
        "A Y" => 4,
        "B X" => 1,
        "C Z" => 7,
        "B Y" => 5,
        "C X" => 2,
        "B Z" => 9,
        "C Y" => 6,
        "A Z" => 8,
        "A X" => 3,
        _ => panic!("{}", play),
    }
}
