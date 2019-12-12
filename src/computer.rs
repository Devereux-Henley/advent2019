//! Computer implementation for problems 2 and 5.
use std::io::{BufRead, Write};

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate
}

impl From<i32> for ParameterMode {
    fn from(value: i32) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid parameter mode.")
        }
    }
}

#[derive(Debug)]
enum OpCode {
    Addition,
    Multiplication,
    Write,
    Output,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals
}

impl OpCode {
    fn instruction_size(&self) -> usize {
        match self {
            OpCode::Addition => 4,
            OpCode::Multiplication => 4,
            OpCode::Write => 2,
            OpCode::Output => 2,
            OpCode::Halt => 1,
            OpCode::JumpIfTrue => 3,
            OpCode::JumpIfFalse => 3,
            OpCode::LessThan => 4,
            OpCode::Equals => 4
        }
    }
}

impl From<i32> for OpCode {
    fn from(opcode: i32) -> Self {
        match opcode {
            1 => OpCode::Addition,
            2 => OpCode::Multiplication,
            3 => OpCode::Write,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Halt,
            _ => panic!("Unexpected Operation Code: [{}]", opcode)
        }
    }
}

#[derive(Debug)]
struct Operation {
    parameter_one_mode: ParameterMode,
    parameter_two_mode: ParameterMode,
    parameter_three_mode: ParameterMode,
    opcode: OpCode
}

impl From<i32> for Operation {
    fn from(instruction: i32) -> Self {
        let mut numeric_state = instruction;
        let mut raw_parameter_mode;

        raw_parameter_mode = numeric_state / 10000;
        let parameter_three_mode = ParameterMode::from(raw_parameter_mode);
        numeric_state -= raw_parameter_mode * 10000;

        raw_parameter_mode = numeric_state / 1000;
        let parameter_two_mode = ParameterMode::from(raw_parameter_mode);
        numeric_state -= raw_parameter_mode * 1000;

        raw_parameter_mode = numeric_state / 100;
        let parameter_one_mode = ParameterMode::from(raw_parameter_mode);
        numeric_state -= raw_parameter_mode * 100;

        Operation {
            parameter_one_mode,
            parameter_two_mode,
            parameter_three_mode,
            opcode: OpCode::from(numeric_state)
        }
    }
}

pub struct Computer<'a, StandardInput: BufRead, StandardOutput: Write> {
    address: usize,
    execution_complete: bool,
    memory: &'a mut [i32],
    stdin: StandardInput,
    stdout: StandardOutput
}

impl<'a, StandardInput: BufRead, StandardOutput: Write> Computer<'a, StandardInput, StandardOutput> {

    pub fn new(memory: &'a mut [i32], stdin: StandardInput, stdout: StandardOutput) -> Computer<'a, StandardInput, StandardOutput> {
        Computer {
            memory,
            address: 0,
            execution_complete: false,
            stdin,
            stdout
        }
    }

    fn advance(&mut self, instruction_size: usize) {
        self.address += instruction_size;
        if self.address >= self.memory.len() {
            self.execution_complete = true;
        }
    }

    fn get_current_operation(&self) -> Operation {
        Operation::from(self.memory[self.address])
    }

    fn get_parameter(&self, address: usize, mode: &ParameterMode) -> i32 {
        match mode {
            ParameterMode::Position => self.memory[self.memory[address] as usize],
            ParameterMode::Immediate => self.memory[address]
        }
    }

    fn process_addition(&mut self, operation: &Operation) -> usize {
        let parameter_one = self.get_parameter(self.address + 1, &operation.parameter_one_mode);
        let parameter_two = self.get_parameter(self.address + 2, &operation.parameter_two_mode);
        let storage_register = self.get_parameter(self.address + 3, &ParameterMode::Immediate) as usize;

        let result = parameter_one + parameter_two;

        self.memory[storage_register] = result;

        operation.opcode.instruction_size()
    }

    fn process_multiplication(&mut self, operation: &Operation) -> usize {
        let parameter_one = self.get_parameter(self.address + 1, &operation.parameter_one_mode);
        let parameter_two = self.get_parameter(self.address + 2, &operation.parameter_two_mode);
        let storage_register = self.get_parameter(self.address + 3, &ParameterMode::Immediate) as usize;

        let result = parameter_one * parameter_two;

        self.memory[storage_register] = result;

        operation.opcode.instruction_size()
    }

    fn process_halt(&mut self, operation: &Operation) -> usize {
        self.execution_complete = true;
        operation.opcode.instruction_size()
    }

    fn process_write(&mut self, operation: &Operation) -> usize {
        let parameter_one = self.get_parameter(self.address + 1, &ParameterMode::Immediate) as usize;

        let mut buffer = String::new();
        self.stdin.read_line(&mut buffer).unwrap();

        let value: i32 = buffer.trim().parse().unwrap();

        self.memory[parameter_one] = value;

        operation.opcode.instruction_size()
    }

    fn process_output(&mut self, operation: &Operation) -> usize {
        let parameter_one = self.get_parameter(self.address + 1, &ParameterMode::Immediate) as usize;

        let value = self.memory[parameter_one];
        let mut output = value.to_string();
        output.push('\n');
        self.stdout.write(output.as_bytes()).unwrap();

        operation.opcode.instruction_size()
    }

    fn process_jump_if_true(&mut self, operation: &Operation) -> usize {
        let parameter_one = self.get_parameter(self.address + 1, &operation.parameter_one_mode);
        let parameter_two = self.get_parameter(self.address + 2, &operation.parameter_two_mode);

        if parameter_one != 0 {
            self.address = parameter_two as usize;
            return 0;
        }

        operation.opcode.instruction_size()
    }

    fn process_jump_if_false(&mut self, operation: &Operation) -> usize {
        let parameter_one = self.get_parameter(self.address + 1, &operation.parameter_one_mode);
        let parameter_two = self.get_parameter(self.address + 2, &operation.parameter_two_mode);

        if parameter_one == 0 {
            self.address = parameter_two as usize;
            return 0;
        }

        return operation.opcode.instruction_size()
    }

    fn process_less_than(&mut self, operation: &Operation) -> usize {
        let parameter_one = self.get_parameter(self.address + 1, &operation.parameter_one_mode);
        let parameter_two = self.get_parameter(self.address + 2, &operation.parameter_two_mode);
        let storage_register = self.get_parameter(self.address + 3, &ParameterMode::Immediate) as usize;

        self.memory[storage_register] = if parameter_one < parameter_two {
            1
        } else {
            0
        };

        operation.opcode.instruction_size()
    }

    fn process_equals(&mut self, operation: &Operation) -> usize {
        let parameter_one = self.get_parameter(self.address + 1, &operation.parameter_one_mode);
        let parameter_two = self.get_parameter(self.address + 2, &operation.parameter_two_mode);
        let storage_register = self.get_parameter(self.address + 3, &ParameterMode::Immediate) as usize;

        self.memory[storage_register] = if parameter_one == parameter_two {
            1
        } else {
            0
        };

        operation.opcode.instruction_size()
    }

    fn process_operation(&mut self, operation: &Operation) -> usize {
        match operation.opcode {
            OpCode::Addition => self.process_addition(operation),
            OpCode::Multiplication => self.process_multiplication(operation),
            OpCode::Write => self.process_write(operation),
            OpCode::Output => self.process_output(operation),
            OpCode::JumpIfTrue => self.process_jump_if_true(operation),
            OpCode::JumpIfFalse => self.process_jump_if_false(operation),
            OpCode::LessThan => self.process_less_than(operation),
            OpCode::Equals => self.process_equals(operation),
            OpCode::Halt => self.process_halt(operation),
        }
    }

    pub fn compute(&mut self) {
        loop {
            if self.execution_complete {
                break;
            }

            let operation = self.get_current_operation();

            let advance_instruction_by = self.process_operation(&operation);

            self.advance(advance_instruction_by);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::computer;

    #[test]
    fn io_one() {
        let buf = BufReader::new("8\n".as_bytes());
        let mut writer: Vec<u8> = vec![];
        let mut data = [3,9,8,9,10,9,4,9,99,-1,8];

        let mut computer = computer::Computer::new(&mut data, buf, &mut writer);

        computer.compute();

        assert_eq!(std::str::from_utf8(writer.as_slice()).unwrap(), "1\n");
    }

    #[test]
    fn io_two() {
        let buf = BufReader::new("7\n".as_bytes());
        let mut writer: Vec<u8> = vec![];
        let mut data = [3,9,7,9,10,9,4,9,99,-1,8];

        let mut computer = computer::Computer::new(&mut data, buf, &mut writer);

        computer.compute();

        assert_eq!(std::str::from_utf8(writer.as_slice()).unwrap(), "1\n");
    }

    #[test]
    fn io_three() {
        let buf = BufReader::new("8\n".as_bytes());
        let mut writer: Vec<u8> = vec![];
        let mut data = [3,3,1108,-1,8,3,4,3,99];

        let mut computer = computer::Computer::new(&mut data, buf, &mut writer);

        computer.compute();

        assert_eq!(std::str::from_utf8(writer.as_slice()).unwrap(), "1\n");
    }

    #[test]
    fn io_four() {
        let buf = BufReader::new("7\n".as_bytes());
        let mut writer: Vec<u8> = vec![];
        let mut data = [3,3,1107,-1,8,3,4,3,99];

        let mut computer = computer::Computer::new(&mut data, buf, &mut writer);

        computer.compute();

        assert_eq!(std::str::from_utf8(writer.as_slice()).unwrap(), "1\n");
    }

    #[test]
    fn jmp_one() {
        let buf = BufReader::new("0\n".as_bytes());
        let mut writer: Vec<u8> = vec![];
        let mut data = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];

        let mut computer = computer::Computer::new(&mut data, buf, &mut writer);

        computer.compute();

        assert_eq!(std::str::from_utf8(writer.as_slice()).unwrap(), "0\n");
    }

    #[test]
    fn jmp_two() {
        let buf = BufReader::new("1\n".as_bytes());
        let mut writer: Vec<u8> = vec![];
        let mut data = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];

        let mut computer = computer::Computer::new(&mut data, buf, &mut writer);

        computer.compute();

        assert_eq!(std::str::from_utf8(writer.as_slice()).unwrap(), "1\n");
    }

    #[test]
    fn jmp_three() {
        let buf = BufReader::new("8\n".as_bytes());
        let mut writer: Vec<u8> = vec![];
        let mut data = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let mut computer = computer::Computer::new(&mut data, buf, &mut writer);

        computer.compute();

        assert_eq!(std::str::from_utf8(writer.as_slice()).unwrap(), "1000\n");
    }
}
