//! Day 6
use std::collections::{HashMap};

pub fn solve_part_one(orbits: Vec<Orbit>) -> i32 {
    let map = InterstellarMap::new(&orbits);

    map.get_orbits()
}

pub fn solve_part_two(orbits: Vec<Orbit>) {
    panic!()
}

#[derive(Debug)]
pub struct Orbit {
    orbitee: String,
    orbiter: String
}

impl<'a> From<String> for Orbit {
    fn from(value: String) -> Self {
        let parts: Vec<&str> = value.split(")").collect();

        Orbit {
            orbitee: parts[0].to_owned(),
            orbiter: parts[1].to_owned()
        }
    }
}

#[derive(Debug)]
struct InterstellarNode {
    orbitees: Vec<usize>
}

struct InterstellarMap {
    nodes: Vec<InterstellarNode>
}

impl InterstellarMap {
    pub fn new(orbits: &Vec<Orbit>) -> Self {
        let mut nodes: Vec<InterstellarNode> = vec![];
        let mut mappings: HashMap<String, usize> = HashMap::new();

        for orbit in orbits {
            if mappings.contains_key(&orbit.orbiter) && mappings.contains_key(&orbit.orbitee) {
                let orbiter_index = mappings.get(&orbit.orbiter).unwrap();
                let orbitee_index = mappings.get(&orbit.orbitee).unwrap();
                nodes[*orbiter_index].orbitees.push(*orbitee_index);
            } else if mappings.contains_key(&orbit.orbiter) {
                let orbitee_index = nodes.len();
                mappings.insert(orbit.orbitee.clone(), orbitee_index);

                nodes.push(InterstellarNode {
                    orbitees: vec![]
                });

                let orbiter_index = mappings.get(&orbit.orbiter).unwrap();
                nodes[*orbiter_index].orbitees.push(orbitee_index);
            } else if mappings.contains_key(&orbit.orbitee) {
                mappings.insert(orbit.orbiter.clone(), nodes.len());

                nodes.push(InterstellarNode {
                    orbitees: vec![*mappings.get(&orbit.orbitee).unwrap()]
                });
            } else {
                let orbitee_index = nodes.len();
                nodes.push(InterstellarNode {
                    orbitees: vec![]
                });

                let orbiter_index = nodes.len();
                nodes.push(InterstellarNode {
                    orbitees: vec![orbitee_index]
                });

                mappings.insert(orbit.orbitee.clone(), orbitee_index);
                mappings.insert(orbit.orbiter.clone(), orbiter_index);
            }
        }

        InterstellarMap {
            nodes
        }
    }

    fn iteratively_get_orbits(&self, orbitees: &Vec<usize>) -> i32 {
        let mut sum = 0;
        let mut current;
        let mut stack = vec![orbitees];

        while stack.len() > 0 {
            current = stack.pop().unwrap();

            for orbitee in current {
                stack.push(&self.nodes[*orbitee].orbitees);
            }

            sum += current.len() as i32;
        }

        sum
    }

    pub fn get_orbits(&self) -> i32 {
        let mut count = 0;

        for node in &self.nodes {
            count += self.iteratively_get_orbits(&node.orbitees)
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::{Orbit, InterstellarMap};

    #[test]
    fn given_one() {
        let orbits = vec![
            Orbit { orbitee: "COM".to_string(), orbiter: "B".to_string() },
            Orbit { orbitee: "B".to_string(), orbiter: "C".to_string() },
            Orbit { orbitee: "C".to_string(), orbiter: "D".to_string() },
            Orbit { orbitee: "D".to_string(), orbiter: "E".to_string() },
            Orbit { orbitee: "E".to_string(), orbiter: "F".to_string() },
            Orbit { orbitee: "B".to_string(), orbiter: "G".to_string() },
            Orbit { orbitee: "G".to_string(), orbiter: "H".to_string() },
            Orbit { orbitee: "D".to_string(), orbiter: "I".to_string() },
            Orbit { orbitee: "E".to_string(), orbiter: "J".to_string() },
            Orbit { orbitee: "J".to_string(), orbiter: "K".to_string() },
            Orbit { orbitee: "K".to_string(), orbiter: "L".to_string() },
        ];

        let map = InterstellarMap::new(&orbits);

        assert_eq!(42, map.get_orbits());
    }
}
