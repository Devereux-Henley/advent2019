mod one;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    solve_one_part_one();
    solve_one_part_two();
}

fn load_day_one_data() -> impl Iterator<Item = i32> {
    let file = File::open("inputs/one.txt").unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().map(|line| {
        let data = line.unwrap();
        data.parse::<i32>().unwrap()
    })
}

fn solve_one_part_one() {
    let stream = load_day_one_data();
    println!("Day 1 Part 1 Solution: {}", one::solve_part_one(stream));
}

fn solve_one_part_two() {
    let stream = load_day_one_data();
    println!("Day 1 Part 2 Solution: {}", one::solve_part_two(stream))
}
