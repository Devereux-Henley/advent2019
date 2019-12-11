mod one;
mod two;
mod three;
mod four;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::convert::TryFrom;

fn main() {
    solve_one_part_one();
    solve_one_part_two();
    solve_two_part_one();
    solve_two_part_two();
    solve_three_part_one();
    solve_three_part_two();
    solve_four_part_one();
    solve_four_part_two();
}

fn load_day_one_data() -> impl Iterator<Item = i32> {
    let file = File::open("inputs/one.txt").unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().map(|line| {
        let data = line.unwrap();
        data.parse::<i32>().unwrap()
    })
}

fn load_day_two_data<'a>() -> Vec<i32> {
    let file = File::open("inputs/two.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut data_line = String::new();
    buf_reader.read_line(&mut data_line).expect("Unable to read day 2 input.");
    data_line.split(",").map(|line| {
        line.parse::<i32>()
    }).filter(|result| result.is_ok()).map(|result| result.unwrap()).collect()
}

fn load_day_three_data() -> Vec<Vec<three::WireVector>> {
    let file = File::open("inputs/three.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.lines().map(|line| {
        line.unwrap()
            .split(",")
            .map(|wire_vector| three::WireVector::try_from(wire_vector))
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap())
            .collect()
    }).collect()
}

fn solve_one_part_one() {
    let stream = load_day_one_data();
    println!("Day 1 Part 1 Solution: {}", one::solve_part_one(stream));
}

fn solve_one_part_two() {
    let stream = load_day_one_data();
    println!("Day 1 Part 2 Solution: {}", one::solve_part_two(stream));
}

fn solve_two_part_one() {
    let mut data = load_day_two_data();
    data[1] = 12;
    data[2] = 2;
    println!("Day 2 Part 1 Solution: {}", two::solve_part_one(&mut data));
}

fn solve_two_part_two() {
    let data = load_day_two_data();
    println!("Day 2 Part 2 Solution: {}", two::solve_part_two(data, 19690720));
}

fn solve_three_part_one() {
    let data = load_day_three_data();
    println!("Day 3 Part 1 Solution: {}", three::solve_part_one(data))
}

fn solve_three_part_two() {
    let data = load_day_three_data();
    println!("Day 3 Part 2 Solution: {}", three::solve_part_two(data))
}

fn solve_four_part_one() {
    println!("Day 4 Part 1 Solution: {}", four::solve_part_one(402328, 864247));
}

fn solve_four_part_two() {
    println!("Day 4 Part 2 Solution: {}", four::solve_part_two(402328, 864247));
}
