//! Day 3
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down
}

#[derive(Debug, Copy, Clone)]
pub struct WireVector {
    pub direction: Direction,
    pub scalar: i32
}

impl TryFrom<&str> for WireVector {
    type Error = &'static str;

    fn try_from(literal: &str) -> Result<Self, Self::Error> {
        let direction = match literal.chars().nth(0) {
            Some('R') => Direction::Right,
            Some('L') => Direction::Left,
            Some('U') => Direction::Up,
            Some('D') => Direction::Down,
            _ => return Err("Bad direction literal.")
        };

        let scalar = literal[1..].parse::<i32>();

        if scalar.is_err() {
            return Err("Failed to parse numeric.");
        }

        Ok(WireVector {
            direction,
            scalar: scalar.unwrap()
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

enum Node {
    Single {
        wire_index: usize,
        step: i32
    },
    Intersection {
        wire_mappings: HashMap<usize, i32>
    }
}

struct Solver {
    cursor: Point,
    step: i32,
    wire_inputs: Vec<Vec<WireVector>>,
    wires: Vec<Vec<Point>>,
    intersections: Vec<Point>,
    point_cache: HashMap<i32, HashMap<i32, Node>>
}

fn calculate_manhattan_distance(point: Point) -> i32 {
    point.x.abs() + point.y.abs()
}

impl Solver {
    pub fn new(inputs: Vec<Vec<WireVector>>) -> Solver {
        Solver {
            wires: vec![],
            step: 0,
            wire_inputs: inputs,
            cursor: Point { x: 0, y: 0 },
            intersections: vec![],
            point_cache: HashMap::new()
        }
    }

    pub fn solve_for_minimum_manhattan_distance(&mut self) -> i32 {
        self.trace_points();
        self.intersections.iter().map(|&point| calculate_manhattan_distance(point)).min().unwrap()
    }

    pub fn solve_for_minimum_step_distance(&mut self) -> i32 {
        self.trace_points();

        let mut smallest_distance = std::i32::MAX;

        for intersection in &self.intersections {
            match self.find_intersection(&intersection) {
                Node::Single { .. } => panic!(),
                Node::Intersection { wire_mappings } => {
                    let sum = wire_mappings.iter().map(|(_, step)| step).sum();
                    if sum < smallest_distance {
                        smallest_distance = sum;
                    }
                }
            }
        }

        smallest_distance
    }

    fn find_intersection(&self, point: &Point) -> &Node {
        self.point_cache.get(&point.x).unwrap().get(&point.y).unwrap()
    }

    fn examine_cursor(&mut self, wire_index: usize) {
        match self.point_cache.get_mut(&self.cursor.x) {
            Some(y_cache) => {
                match y_cache.get_mut(&self.cursor.y) {
                    Some(node) => {
                        match node {
                            Node::Single { wire_index: existing_wire_index, step: existing_steps } => {
                                if wire_index == *existing_wire_index {
                                    return;
                                }

                                let mut wire_mappings = HashMap::new();
                                wire_mappings.insert(wire_index, self.step);
                                wire_mappings.insert(*existing_wire_index, *existing_steps);
                                let new_intersection = Node::Intersection { wire_mappings };
                                y_cache.insert(self.cursor.y, new_intersection);
                                self.intersections.push(self.cursor);
                            },
                            Node::Intersection{ wire_mappings } => {
                                if wire_mappings.contains_key(&wire_index) {
                                    return;
                                }

                                wire_mappings.insert(wire_index, self.step);
                            }
                        }
                    },
                    None => {
                        y_cache.insert(self.cursor.y, Node::Single{ wire_index, step: self.step });
                    }
                }
            },
            None => {
                let mut y_cache: HashMap<i32, Node> = HashMap::new();
                y_cache.insert(self.cursor.y, Node::Single{ wire_index, step: self.step });
                self.point_cache.insert(self.cursor.x, y_cache);
            }
        }
    }

    fn place_point(&mut self, wire_index: usize) {
        self.wires[wire_index].push(self.cursor);
    }

    fn perform_step(&mut self, wire_index: usize) {
        self.step += 1;
        self.examine_cursor(wire_index);
        self.place_point(wire_index);
    }

    fn trace_right(&mut self, scalar: i32, wire_index: usize) {
        for _ in 0..scalar {
            self.cursor.x += 1;
            self.perform_step(wire_index);
        }
    }

    fn trace_left(&mut self, scalar: i32, wire_index: usize) {
        for _ in 0..scalar {
            self.cursor.x -= 1;
            self.perform_step(wire_index);
        }
    }

    fn trace_up(&mut self, scalar: i32, wire_index: usize) {
        for _ in 0..scalar {
            self.cursor.y += 1;
            self.perform_step(wire_index);
        }
    }

    fn trace_down(&mut self, scalar: i32, wire_index: usize) {
        for _ in 0..scalar {
            self.cursor.y -= 1;
            self.perform_step(wire_index);
        }
    }

    fn trace_points(&mut self) {
        for wire_index in 0..self.wire_inputs.len() {
            self.cursor.x = 0;
            self.cursor.y = 0;
            self.wires.push(vec![]);
            self.step = 0;
            for vector_index in 0..self.wire_inputs[wire_index].len() {
                let WireVector { direction, scalar } = self.wire_inputs[wire_index][vector_index];

                match direction {
                    Direction::Right => self.trace_right(scalar, wire_index),
                    Direction::Left => self.trace_left(scalar, wire_index),
                    Direction::Up => self.trace_up(scalar, wire_index),
                    Direction::Down => self.trace_down(scalar, wire_index)
                }
            }
        }
    }
}

pub fn solve_part_one(wires: Vec<Vec<WireVector>>) -> i32 {
    let mut solver = Solver::new(wires);

    solver.solve_for_minimum_manhattan_distance()
}

pub fn solve_part_two(wires: Vec<Vec<WireVector>>) -> i32 {
    let mut solver = Solver::new(wires);

    solver.solve_for_minimum_step_distance()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one_given_one() {
        let wire_one_directions = vec![
            super::WireVector { direction: super::Direction::Right, scalar: 75 },
            super::WireVector { direction: super::Direction::Down, scalar: 30 },
            super::WireVector { direction: super::Direction::Right, scalar: 83 },
            super::WireVector { direction: super::Direction::Up, scalar: 83 },
            super::WireVector { direction: super::Direction::Left, scalar: 12 },
            super::WireVector { direction: super::Direction::Down, scalar: 49 },
            super::WireVector { direction: super::Direction::Right, scalar: 71 },
            super::WireVector { direction: super::Direction::Up, scalar: 7 },
            super::WireVector { direction: super::Direction::Left, scalar: 72 }
        ];

        let wire_two_directions = vec![
            super::WireVector { direction: super::Direction::Up, scalar: 62 },
            super::WireVector { direction: super::Direction::Right, scalar: 66 },
            super::WireVector { direction: super::Direction::Up, scalar: 55 },
            super::WireVector { direction: super::Direction::Right, scalar: 34 },
            super::WireVector { direction: super::Direction::Down, scalar: 71 },
            super::WireVector { direction: super::Direction::Right, scalar: 55 },
            super::WireVector { direction: super::Direction::Down, scalar: 58 },
            super::WireVector { direction: super::Direction::Right, scalar: 83 }
        ];

        assert_eq!(159, super::solve_part_one(vec![wire_one_directions, wire_two_directions]));
    }

    #[test]
    fn part_one_given_two() {
        let wire_one_directions = vec![
            super::WireVector { direction: super::Direction::Right, scalar: 98 },
            super::WireVector { direction: super::Direction::Up, scalar: 47 },
            super::WireVector { direction: super::Direction::Right, scalar: 26 },
            super::WireVector { direction: super::Direction::Down, scalar: 63 },
            super::WireVector { direction: super::Direction::Right, scalar: 33 },
            super::WireVector { direction: super::Direction::Up, scalar: 87 },
            super::WireVector { direction: super::Direction::Left, scalar: 62 },
            super::WireVector { direction: super::Direction::Down, scalar: 20 },
            super::WireVector { direction: super::Direction::Right, scalar: 33 },
            super::WireVector { direction: super::Direction::Up, scalar: 53 },
            super::WireVector { direction: super::Direction::Right, scalar: 51 }
        ];

        let wire_two_directions = vec![
            super::WireVector { direction: super::Direction::Up, scalar: 98 },
            super::WireVector { direction: super::Direction::Right, scalar: 91 },
            super::WireVector { direction: super::Direction::Down, scalar: 20 },
            super::WireVector { direction: super::Direction::Right, scalar: 16 },
            super::WireVector { direction: super::Direction::Down, scalar: 67 },
            super::WireVector { direction: super::Direction::Right, scalar: 40 },
            super::WireVector { direction: super::Direction::Up, scalar: 7 },
            super::WireVector { direction: super::Direction::Right, scalar: 15 },
            super::WireVector { direction: super::Direction::Up, scalar: 6 },
            super::WireVector { direction: super::Direction::Right, scalar: 7 }
        ];

        assert_eq!(135, super::solve_part_one(vec![wire_one_directions, wire_two_directions]));
    }

    #[test]
    fn part_two_given_one() {
        let wire_one_directions = vec![
            super::WireVector { direction: super::Direction::Right, scalar: 75 },
            super::WireVector { direction: super::Direction::Down, scalar: 30 },
            super::WireVector { direction: super::Direction::Right, scalar: 83 },
            super::WireVector { direction: super::Direction::Up, scalar: 83 },
            super::WireVector { direction: super::Direction::Left, scalar: 12 },
            super::WireVector { direction: super::Direction::Down, scalar: 49 },
            super::WireVector { direction: super::Direction::Right, scalar: 71 },
            super::WireVector { direction: super::Direction::Up, scalar: 7 },
            super::WireVector { direction: super::Direction::Left, scalar: 72 }
        ];

        let wire_two_directions = vec![
            super::WireVector { direction: super::Direction::Up, scalar: 62 },
            super::WireVector { direction: super::Direction::Right, scalar: 66 },
            super::WireVector { direction: super::Direction::Up, scalar: 55 },
            super::WireVector { direction: super::Direction::Right, scalar: 34 },
            super::WireVector { direction: super::Direction::Down, scalar: 71 },
            super::WireVector { direction: super::Direction::Right, scalar: 55 },
            super::WireVector { direction: super::Direction::Down, scalar: 58 },
            super::WireVector { direction: super::Direction::Right, scalar: 83 }
        ];

        assert_eq!(610, super::solve_part_two(vec![wire_one_directions, wire_two_directions]));
    }

    #[test]
    fn part_two_given_two() {
        let wire_one_directions = vec![
            super::WireVector { direction: super::Direction::Right, scalar: 98 },
            super::WireVector { direction: super::Direction::Up, scalar: 47 },
            super::WireVector { direction: super::Direction::Right, scalar: 26 },
            super::WireVector { direction: super::Direction::Down, scalar: 63 },
            super::WireVector { direction: super::Direction::Right, scalar: 33 },
            super::WireVector { direction: super::Direction::Up, scalar: 87 },
            super::WireVector { direction: super::Direction::Left, scalar: 62 },
            super::WireVector { direction: super::Direction::Down, scalar: 20 },
            super::WireVector { direction: super::Direction::Right, scalar: 33 },
            super::WireVector { direction: super::Direction::Up, scalar: 53 },
            super::WireVector { direction: super::Direction::Right, scalar: 51 }
        ];

        let wire_two_directions = vec![
            super::WireVector { direction: super::Direction::Up, scalar: 98 },
            super::WireVector { direction: super::Direction::Right, scalar: 91 },
            super::WireVector { direction: super::Direction::Down, scalar: 20 },
            super::WireVector { direction: super::Direction::Right, scalar: 16 },
            super::WireVector { direction: super::Direction::Down, scalar: 67 },
            super::WireVector { direction: super::Direction::Right, scalar: 40 },
            super::WireVector { direction: super::Direction::Up, scalar: 7 },
            super::WireVector { direction: super::Direction::Right, scalar: 15 },
            super::WireVector { direction: super::Direction::Up, scalar: 6 },
            super::WireVector { direction: super::Direction::Right, scalar: 7 }
        ];

        assert_eq!(410, super::solve_part_two(vec![wire_one_directions, wire_two_directions]));
    }
}
