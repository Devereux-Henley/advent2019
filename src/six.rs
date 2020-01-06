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

impl<'a> From<String> for Orbit {
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
    orbitees: Vec<usize>,
    orbiters: Vec<usize>
}

struct InterstellarMap {
    nodes: Vec<InterstellarNode>,
    starmap: HashMap<String, usize>
}

impl InterstellarMap {
    pub fn new(orbits: &[Orbit]) -> Self {
        let mut nodes: Vec<InterstellarNode> = vec![];
        let mut mappings: HashMap<String, usize> = HashMap::new();

        for orbit in orbits {
            if mappings.contains_key(&orbit.orbiter) && mappings.contains_key(&orbit.orbitee) {
                let orbiter_index = mappings.get(&orbit.orbiter).unwrap();
                let orbitee_index = mappings.get(&orbit.orbitee).unwrap();
                nodes[*orbiter_index].orbitees.push(*orbitee_index);
                nodes[*orbitee_index].orbiters.push(*orbiter_index);
            } else if mappings.contains_key(&orbit.orbiter) {
                let orbitee_index = nodes.len();
                let orbiter_index = *mappings.get(&orbit.orbiter).unwrap();
                mappings.insert(orbit.orbitee.clone(), orbitee_index);

                nodes.push(InterstellarNode {
                    orbitees: vec![],
                    orbiters: vec![orbiter_index]
                });

                nodes[orbiter_index].orbitees.push(orbitee_index);
            } else if mappings.contains_key(&orbit.orbitee) {
                let orbiter_index = nodes.len();
                let orbitee_index = *mappings.get(&orbit.orbitee).unwrap();

                mappings.insert(orbit.orbiter.clone(), nodes.len());

                nodes.push(InterstellarNode {
                    orbitees: vec![*mappings.get(&orbit.orbitee).unwrap()],
                    orbiters: vec![]
                });

                nodes[orbitee_index].orbiters.push(orbiter_index);
            } else {
                let orbitee_index = nodes.len();
                let orbiter_index = orbitee_index + 1;
                nodes.push(InterstellarNode {
                    orbitees: vec![],
                    orbiters: vec![orbiter_index]
                });

                nodes.push(InterstellarNode {
                    orbitees: vec![orbitee_index],
                    orbiters: vec![]
                });

                mappings.insert(orbit.orbitee.clone(), orbitee_index);
                mappings.insert(orbit.orbiter.clone(), orbiter_index);
            }
        }

        InterstellarMap {
            nodes,
            starmap: mappings
        }
    }

    fn iteratively_get_orbits(&self, orbitees: &[usize]) -> i32 {
        let mut sum = 0;
        let mut current;
        let mut stack = vec![orbitees];

        while !stack.is_empty() {
            current = stack.pop().unwrap();

            for orbitee in current {
                stack.push(&self.nodes[*orbitee].orbitees);
            }

            sum += current.len() as i32;
        }

        sum
    }

    // Dijkstra implementation for shortest path.
    pub fn find_shortest_path(&self, node_one_name: &str, node_two_name: &str) -> i32 {
        let node_one = *self.starmap.get(node_one_name).unwrap();
        let node_two = *self.starmap.get(node_two_name).unwrap();
        let mut distances: HashMap<usize, i32> = HashMap::new();
        let mut previous: HashMap<usize, usize> = HashMap::new();

        distances.insert(node_one, 0);

        let mut alt;
        let mut set: HashSet<usize> = (0..self.nodes.len()).collect();

        // Evaluate every node once.
        while !set.is_empty()  {

            // Find a node with smallest distance.
            let node_index = *set.iter().fold(None, |acc_option, key_in_set| {
                if acc_option.is_none() {
                    return Some(key_in_set);
                }

                let acc = acc_option.unwrap();

                Some(
                    match distances.get(&acc) {
                        Some(value) => match distances.get(&key_in_set) {
                            Some(acc_value) => if value < acc_value { key_in_set } else { acc },
                            None => acc
                        },
                        None => key_in_set
                    })
            }).unwrap();

            // Remove this node from further evaluation.
            set.remove(&node_index);

            let node = &self.nodes[node_index];

            // Update distances for all orbitees.
            for orbitee in &node.orbitees {
                alt = match distances.get(&node_index) {
                    Some(value) => value + 1,
                    None => panic!("This shouldn't happen: {}, {:?}", node_index, distances)
                };

                let compare = match distances.get(&orbitee) {
                    Some(value) => *value,
                    None => std::i32::MAX
                };

                if alt < compare {
                    distances.insert(*orbitee, alt);
                    previous.insert(*orbitee, node_index);
                }
            }

            // Update distances for all orbiters.
            for orbiter in &node.orbiters {
                alt = match distances.get(&node_index) {
                    Some(value) => value + 1,
                    None => panic!("This shouldn't happen: {}, {:?}", node_index, distances)
                };

                let compare = match distances.get(&orbiter) {
                    Some(value) => *value,
                    None => std::i32::MAX
                };

                if alt < compare {
                    distances.insert(*orbiter, alt);
                    previous.insert(*orbiter, node_index);
                }
            }
        }

        let mut target = &node_two;
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
