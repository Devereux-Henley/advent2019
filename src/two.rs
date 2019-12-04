//! Day 2
pub fn solve_part_one(input: &mut [i32]) -> i32 {
    let mut calculator = Calculator {
        register: input,
        step: 0,
        execution_complete: false
    };

    calculator.calculate();

    input[0]
}

struct Calculator<'a> {
    step: usize,
    execution_complete: bool,
    register: &'a mut [i32]
}

impl<'a> Calculator<'a> {
    fn advance(&mut self) {
        self.step += 4;
        if self.step >= self.register.len() {
            self.execution_complete = true;
        }
    }

    fn get_current_opcode(&self) -> i32 {
        self.register[self.step]
    }

    fn process_addition(&mut self) {
        assert!(self.register.len() > self.step + 3);

        let register_one = self.register[self.step + 1] as usize;
        let register_two = self.register[self.step + 2] as usize;
        let storage_register = self.register[self.step + 3] as usize;

        assert!(self.register.len() > storage_register as usize);

        let result = self.register[register_one] + self.register[register_two];

        self.register[storage_register] = result;
    }

    fn process_multiplication(&mut self) {
        assert!(self.register.len() > self.step + 3);

        let register_one = self.register[self.step + 1] as usize;
        let register_two = self.register[self.step + 2] as usize;
        let storage_register = self.register[self.step + 3] as usize;

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

    pub fn calculate(&mut self) {
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
        assert_eq!([2, 0, 0, 0, 99], super::solve_part_one(&mut[1, 0, 0, 0, 99]))
    }

    #[test]
    fn given_two() {
        assert_eq!([2, 3, 0, 6, 99], super::solve_part_one(&mut[2, 3, 0, 3, 99]))
    }

    #[test]
    fn given_three() {
        assert_eq!([2, 4, 4, 5, 99, 9801], super::solve_part_one(&mut[2, 4, 4, 5, 99, 0]))
    }

    #[test]
    fn given_four() {
        assert_eq!([30, 1, 1, 4, 2, 5, 6, 0, 99], super::solve_part_one(&mut[1, 1, 1, 4, 99, 5, 6, 0, 99]))
    }
}