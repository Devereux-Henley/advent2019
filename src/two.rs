//! Day 2
pub fn solve_part_one(input: &mut [i32]) -> i32 {
    let mut computer = Computer {
        register: input,
        address: 0,
        execution_complete: false
    };

    computer.compute();

    input[0]
}

pub fn solve_part_two(input: Vec<i32>, target: i32) -> i32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut copied_input = input.clone();

            copied_input[1] = noun;
            copied_input[2] = verb;

            let mut computer = Computer {
                register: &mut copied_input,
                address: 0,
                execution_complete: false
            };

            computer.compute();

            let result = copied_input[0];

            if result == target {
                return 100 * noun + verb;
            }
        }
    }

    panic!();
}

struct Computer<'a> {
    address: usize,
    execution_complete: bool,
    register: &'a mut [i32]
}

impl<'a> Computer<'a> {
    fn advance(&mut self) {
        self.address += 4;
        if self.address >= self.register.len() {
            self.execution_complete = true;
        }
    }

    fn get_current_opcode(&self) -> i32 {
        self.register[self.address]
    }

    fn process_addition(&mut self) {
        assert!(self.register.len() > self.address + 3);

        let register_one = self.register[self.address + 1] as usize;
        let register_two = self.register[self.address + 2] as usize;
        let storage_register = self.register[self.address + 3] as usize;

        assert!(self.register.len() > storage_register as usize);

        let result = self.register[register_one] + self.register[register_two];

        self.register[storage_register] = result;
    }

    fn process_multiplication(&mut self) {
        assert!(self.register.len() > self.address + 3);

        let register_one = self.register[self.address + 1] as usize;
        let register_two = self.register[self.address + 2] as usize;
        let storage_register = self.register[self.address + 3] as usize;

        assert!(self.register.len() > storage_register as usize);

        let result = self.register[register_one] * self.register[register_two];

        self.register[storage_register] = result;
    }

    fn process_halt(&mut self) {
        self.execution_complete = true;
    }

    fn process_opcode(&mut self, opcode: i32) {
        match opcode {
            1 => self.process_addition(),
            2 => self.process_multiplication(),
            99 => self.process_halt(),
            _ => panic!()
        }
    }

    pub fn compute(&mut self) {
        loop {
            if self.execution_complete {
                break;
            }

            let opcode = self.get_current_opcode();

            self.process_opcode(opcode);

            self.advance();
        }
    }
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
