//! Day 6
use std::collections::{HashMap, HashSet};

pub fn solve_part_one(orbits: Vec<Orbit>) -> i32 {
    let map = InterstellarMap::new(&orbits);

    map.get_orbits()
}

pub fn solve_part_two(orbits: Vec<Orbit>) -> i32 {
    let map = InterstellarMap::new(&orbits);

    map.find_shortest_path("YOU", "SAN")
}

#[derive(Debug)]
pub struct Orbit {
    orbitee: String,
    orbiter: String
}

impl From<String> for Orbit {
    fn from(value: String) -> Self {
        let parts: Vec<&str> = value.split(')').collect();

        Orbit {
            orbitee: parts[0].to_owned(),
            orbiter: parts[1].to_owned()
        }
    }
}

#[derive(Debug)]
struct InterstellarNode {
    orbitees: Vec<String>,
    orbiters: Vec<String>
}

struct InterstellarMap {
    starmap: HashMap<String, InterstellarNode>
}

impl InterstellarMap {
    pub fn new(orbits: &[Orbit]) -> Self {
        let mut mappings: HashMap<String, InterstellarNode> = HashMap::new();

        for orbit in orbits {
            if mappings.contains_key(&orbit.orbiter) && mappings.contains_key(&orbit.orbitee) {
                let orbiter = mappings.get_mut(&orbit.orbiter).unwrap();
                orbiter.orbitees.push(orbit.orbitee.clone());

                let orbitee = mappings.get_mut(&orbit.orbitee).unwrap();
                orbitee.orbiters.push(orbit.orbiter.clone());
            } else if mappings.contains_key(&orbit.orbiter) {
                mappings.insert(orbit.orbitee.clone(), InterstellarNode {
                    orbitees: vec![],
                    orbiters: vec![orbit.orbiter.clone()]
                });

                let orbiter = mappings.get_mut(&orbit.orbiter).unwrap();
                orbiter.orbitees.push(orbit.orbitee.clone());
            } else if mappings.contains_key(&orbit.orbitee) {
                mappings.insert(orbit.orbiter.clone(), InterstellarNode {
                    orbitees: vec![orbit.orbitee.clone()],
                    orbiters: vec![]
                });

                let orbitee = mappings.get_mut(&orbit.orbitee).unwrap();
                orbitee.orbiters.push(orbit.orbiter.clone());
            } else {
                mappings.insert(orbit.orbitee.clone(), InterstellarNode {
                    orbitees: vec![],
                    orbiters: vec![orbit.orbiter.clone()]
                });

                mappings.insert(orbit.orbiter.clone(), InterstellarNode {
                    orbitees: vec![orbit.orbitee.clone()],
                    orbiters: vec![]
                });
            }
        }

        InterstellarMap {
            starmap: mappings
        }
    }

    fn iteratively_get_orbits(&self, orbitees: &[String]) -> i32 {
        let mut sum = 0;
        let mut current;
        let mut stack = vec![orbitees];

        while !stack.is_empty() {
            current = stack.pop().unwrap();

            for orbitee in current {
                stack.push(&self.starmap[orbitee].orbitees);
            }

            sum += current.len() as i32;
        }

        sum
    }

    // Dijkstra implementation for shortest path.
    pub fn find_shortest_path(&self, node_one_name: &str, node_two_name: &str) -> i32 {
        let mut distances: HashMap<String, i32> = HashMap::new();
        let mut previous: HashMap<String, String> = HashMap::new();

        distances.insert(node_one_name.to_string(), 0);

        let mut alt;
        let mut set: HashSet<String> = self.starmap.iter().map(|(key, _)| key.clone()).collect();

        // Evaluate every node once.
        while !set.is_empty()  {

            // Find a node with smallest distance.
            let mut node_key_ref: Option<&String> = None;

            for key in &set {
                if node_key_ref.is_none() {
                    node_key_ref = Some(key);
                }

                let node = node_key_ref.unwrap();

                match distances.get(node) {
                    Some(value) => match distances.get(key) {
                        Some(acc_value) => if value < acc_value { node_key_ref = Some(key) },
                        None => ()
                    },
                    None => node_key_ref = Some(key)
                }
            }

            let node_key: &String = node_key_ref.unwrap();

            let node = &self.starmap.get(node_key).unwrap();

            // Update distances for all orbitees.
            for orbitee in &node.orbitees {
                alt = match distances.get(node_key) {
                    Some(value) => value + 1,
                    None => panic!("This shouldn't happen: {}, {:?}", node_key, distances)
                };

                let compare = match distances.get(orbitee) {
                    Some(value) => *value,
                    None => std::i32::MAX
                };

                if alt < compare {
                    distances.insert(orbitee.clone(), alt);
                    previous.insert(orbitee.clone(), node_key.clone());
                }
            }

            // Update distances for all orbiters.
            for orbiter in &node.orbiters {
                alt = match distances.get(node_key) {
                    Some(value) => value + 1,
                    None => panic!("This shouldn't happen: {}, {:?}", node_key, distances)
                };

                let compare = match distances.get(orbiter) {
                    Some(value) => *value,
                    None => std::i32::MAX
                };

                if alt < compare {
                    distances.insert(orbiter.clone(), alt);
                    previous.insert(orbiter.clone(), node_key.clone());
                }
            }

            // Remove this node from further evaluation.
            let set_key = node_key.clone();
            set.remove(&set_key);
        }

        let mut target = node_two_name;
        let mut distance = 0;

        // If we did not pass the node, it is disconnected and cannot be reached.
        if !previous.contains_key(target) {
            panic!("Unreachable target node.");
        }

        // Update distance for every node in the chain.
        while let Some(node) = previous.get(target) {
            distance += 1;
            target = node
        }

        // Subtract start and end node from chain for minimal orbital transfers.
        distance - 2
    }

    pub fn get_orbits(&self) -> i32 {
        let mut count = 0;

        for (_, node) in self.starmap.iter() {
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

    #[test]
    fn given_two() {
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
            Orbit { orbitee: "K".to_string(), orbiter: "YOU".to_string() },
            Orbit { orbitee: "I".to_string(), orbiter: "SAN".to_string() },
        ];

        let map = InterstellarMap::new(&orbits);

        assert_eq!(4, map.find_shortest_path("YOU", "SAN"));
    }
}
