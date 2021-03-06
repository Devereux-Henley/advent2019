//! Day 4

pub fn solve_part_one(lower_bound: i32, upper_bound: i32) -> i32 {
    let mut accumulator = 0;
    for value in lower_bound..upper_bound {
        if check_adjacency(value) && check_increasing(value) {
            accumulator += 1
        }
    }

    accumulator
}

pub fn solve_part_two(lower_bound: i32, upper_bound: i32) -> i32 {
    let mut accumulator = 0;
    for value in lower_bound..upper_bound {
        if check_double_adjacency(value) && check_increasing(value) {
            accumulator += 1
        }
    }

    accumulator
}

fn check_adjacency(numeric: i32) -> bool {
    let mut div_state = 100_000;
    let mut adjacency_state =  numeric / div_state;
    let mut numeric_state = numeric - div_state * adjacency_state;
    while div_state > 1 {
        div_state /= 10;
        let next_adjacency = numeric_state / div_state;
        if adjacency_state == next_adjacency {
            return true;
        }

        adjacency_state = next_adjacency;
        numeric_state -= div_state * adjacency_state;
    }

    false
}

enum Adjacency {
    Single,
    Double,
    Multiple
}

struct AdjacencyState {
    state: Adjacency,
    value: i32
}

fn check_double_adjacency(numeric: i32) -> bool {
    let mut div_state = 100_000;
    let mut adjacency_state = AdjacencyState { state: Adjacency::Single, value: numeric / div_state };
    let mut numeric_state = numeric - div_state * adjacency_state.value;
    while div_state > 1 {
        div_state /= 10;
        let next_adjacency = numeric_state / div_state;
        if adjacency_state.value == next_adjacency {
            match adjacency_state.state {
                Adjacency::Single => adjacency_state.state = Adjacency::Double,
                Adjacency::Double => adjacency_state.state = Adjacency::Multiple,
                Adjacency::Multiple => ()
            }
        } else {
            match adjacency_state.state {
                Adjacency::Double => return true,
                _ => adjacency_state.state = Adjacency::Single
            }
        }

        adjacency_state.value = next_adjacency;
        numeric_state -= div_state * adjacency_state.value;
    }

    match adjacency_state.state {
        Adjacency::Double => true,
        _ => false
    }
}

fn check_increasing(numeric: i32) -> bool {
    let mut div_state = 100_000;
    let mut adjacency_state =  numeric / div_state;
    let mut numeric_state = numeric - div_state * adjacency_state;
    while div_state > 1 {
        div_state /= 10;
        let next_adjacency = numeric_state / div_state;
        if adjacency_state > next_adjacency {
            return false;
        }

        adjacency_state = next_adjacency;
        numeric_state -= div_state * adjacency_state;
    }

    true
}

#[cfg(test)]
mod tests {

    #[test]
    fn check_adjacency() {
        assert!(super::check_adjacency(110_000));
        assert!(super::check_adjacency(120_012));
        assert!(super::check_adjacency(001_234));
    }

    #[test]
    fn check_double_adjacency() {
        assert!(super::check_double_adjacency(112_345));
        assert!(super::check_double_adjacency(111_122));
        assert!(super::check_double_adjacency(001_112));
    }

    #[test]
    fn check_non_double_adjacency() {
        assert!(!super::check_double_adjacency(111345));
        assert!(!super::check_double_adjacency(111222));
        assert!(!super::check_double_adjacency(000000));
    }

    #[test]
    fn check_non_adjacency() {
        assert!(!super::check_adjacency(012345));
        assert!(!super::check_adjacency(123456));
    }

    #[test]
    fn check_increasing() {
        assert!(super::check_increasing(123456));
    }

    #[test]
    fn check_non_increasing() {
        assert!(!super::check_increasing(654321));
        assert!(!super::check_increasing(543210));
        assert!(!super::check_increasing(799990));
    }
}
