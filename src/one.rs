//! Day 1
use std::iter::Iterator;

pub fn solve_part_one(input: impl Iterator<Item = i32>) -> i32 {
    input.map(find_fuel_for_mass).sum()
}

fn find_fuel_for_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

fn find_fuel_for_mass_and_fuel(mass: i32) -> i32 {
    let mut total_fuel_mass = find_fuel_for_mass(mass);
    let mut next_fuel_mass = total_fuel_mass;

    loop {
        if next_fuel_mass == 0 {
            break;
        }

        next_fuel_mass = zero_negatives(find_fuel_for_mass(next_fuel_mass));
        total_fuel_mass += next_fuel_mass
    }

    total_fuel_mass
}

fn zero_negatives(value: i32) -> i32 {
    if value > 0 {
        value
    } else {
        0
    }
}

pub fn solve_part_two(input: impl Iterator<Item = i32>) -> i32 {
    input.map(find_fuel_for_mass_and_fuel).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_one() {
        assert_eq!(super::find_fuel_for_mass(12), 2)
    }

    #[test]
    fn given_two() {
        assert_eq!(super::find_fuel_for_mass(14), 2)
    }

    #[test]
    fn given_three() {
        assert_eq!(super::find_fuel_for_mass(1969), 654)
    }

    #[test]
    fn given_four() {
        assert_eq!(super::find_fuel_for_mass(100756), 33583)
    }
}
