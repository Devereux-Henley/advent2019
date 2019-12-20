mod computer;
mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::convert::TryFrom;

fn main() {
    solve_six_part_one();
    solve_six_part_two();
}

fn load_day_one_data() -> impl Iterator<Item = i32> {
    let file = File::open("inputs/one.txt").unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().map(|line| {
        let data = line.unwrap();
        data.parse::<i32>().unwrap()
    })
}

fn load_day_two_data() -> Vec<i32> {
    let file = File::open("inputs/two.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut data_line = String::new();
    buf_reader.read_line(&mut data_line).expect("Unable to read day 2 input.");
    data_line.trim().split(',').map(|line| {
        line.parse::<i32>()
    }).filter(|result| result.is_ok()).map(|result| result.unwrap()).collect()
}

fn load_day_three_data() -> Vec<Vec<three::WireVector>> {
    let file = File::open("inputs/three.txt").unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().map(|line| {
        line.unwrap()
            .split(',')
            .map(three::WireVector::try_from)
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap())
            .collect()
    }).collect()
}

fn load_day_five_data() -> Vec<i32> {
    let file = File::open("inputs/five.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut data_line = String::new();
    buf_reader.read_line(&mut data_line).expect("Unable to read day 5 input.");
    data_line.trim().split(',').map(|line| {
        line.parse::<i32>()
    }).filter(|result| result.is_ok()).map(|result| result.unwrap()).collect()
}

fn load_day_six_data() -> Vec<six::Orbit> {
    let file = File::open("inputs/six.txt").unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().map(|line| {
        six::Orbit::from(line.unwrap())
    }).collect()
}

#[allow(dead_code)]
fn solve_one_part_one() {
    let stream = load_day_one_data();
    println!("Day 1 Part 1 Solution: {}", one::solve_part_one(stream));
}

#[allow(dead_code)]
fn solve_one_part_two() {
    let stream = load_day_one_data();
    println!("Day 1 Part 2 Solution: {}", one::solve_part_two(stream));
}

#[allow(dead_code)]
fn solve_two_part_one() {
    let mut data = load_day_two_data();
    data[1] = 12;
    data[2] = 2;
    println!("Day 2 Part 1 Solution: {}", two::solve_part_one(&mut data));
}

#[allow(dead_code)]
fn solve_two_part_two() {
    let data = load_day_two_data();
    println!("Day 2 Part 2 Solution: {}", two::solve_part_two(data, 19_690_720));
}

#[allow(dead_code)]
fn solve_three_part_one() {
    let data = load_day_three_data();
    println!("Day 3 Part 1 Solution: {}", three::solve_part_one(data))
}

#[allow(dead_code)]
fn solve_three_part_two() {
    let data = load_day_three_data();
    println!("Day 3 Part 2 Solution: {}", three::solve_part_two(data))
}

#[allow(dead_code)]
fn solve_four_part_one() {
    println!("Day 4 Part 1 Solution: {}", four::solve_part_one(402_328, 864_247));
}

#[allow(dead_code)]
fn solve_four_part_two() {
    println!("Day 4 Part 2 Solution: {}", four::solve_part_two(402_328, 864_247));
}

#[allow(dead_code)]
fn solve_five() {
    let mut data = load_day_five_data();
    five::solve_part_one(&mut data);
}

#[allow(dead_code)]
fn solve_six_part_one() {
    let data = load_day_six_data();
    println!("Day 6 Part 1 Solution: {}", six::solve_part_one(data));
}

#[allow(dead_code)]
fn solve_six_part_two() {
    let data = load_day_six_data();
    println!("Day 6 Part 2 Solution: {}", six::solve_part_two(data));
}
