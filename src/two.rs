//! Day 2
use crate::computer;
use std::io::{self};

pub fn solve_part_one(input: &mut [i32]) -> i32 {
    let stdin = io::stdin();
    let mut computer = computer::Computer::new(input, stdin.lock(), io::stdout());

    computer.compute();

    input[0]
}

pub fn solve_part_two(input: Vec<i32>, target: i32) -> i32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut copied_input = input.clone();

            copied_input[1] = noun;
            copied_input[2] = verb;

            let stdin = io::stdin();
            let mut computer = computer::Computer::new(&mut copied_input, stdin.lock(), io::stdout());

            computer.compute();

            let result = copied_input[0];

            if result == target {
                return 100 * noun + verb;
            }
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_one() {
        assert_eq!(2, super::solve_part_one(&mut[1, 0, 0, 0, 99]))
    }

    #[test]
    fn given_two() {
        assert_eq!(2, super::solve_part_one(&mut[2, 3, 0, 3, 99]))
    }

    #[test]
    fn given_three() {
        assert_eq!(2, super::solve_part_one(&mut[2, 4, 4, 5, 99, 0]))
    }

    #[test]
    fn given_four() {
        assert_eq!(30, super::solve_part_one(&mut[1, 1, 1, 4, 99, 5, 6, 0, 99]))
    }
}
