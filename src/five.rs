//! Day 5
use crate::computer;
use std::io::{self};

pub fn solve_part_one(input: &mut [i32]) {
    let stdin = io::stdin();
    let mut computer = computer::Computer::new(input, stdin.lock(), io::stdout());

    computer.compute();
}
