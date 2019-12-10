// Intcode processig module
mod intcode {
    // Imports
    use std::convert::TryFrom;

    // Private (helpers)
    #[derive(Eq,PartialEq)]
    enum ParameterMode {
        POSITION,
        IMMEDIATE,
        RELATIVE
    }
    impl TryFrom<(isize, u8)> for ParameterMode {
        type Error = String;
        fn try_from(modes_pos: (isize, u8)) -> Result<Self, Self::Error> {
            match modes_pos.0 / (10_isize.pow(modes_pos.1 as u32)) % 10 {
                0 => Ok(Self::POSITION),
                1 => Ok(Self::IMMEDIATE),
                2 => Ok(Self::RELATIVE),
                _ => Err( format!("Invalid parameter mode at {} in {}!", modes_pos.1, modes_pos.0) )
            }
        }
    }

    // Public
    pub enum IntcodeResult {
        OUTPUT(isize),
        HALT
    }

    pub struct Processor {
        intcode: Vec<isize>,
        position: usize,
        relative_base: usize,
        input: Vec<isize>
    }
    impl Processor {
        // State
        pub fn new(intcode: Vec<isize>, input: Vec<isize>) -> Self {
            Self {
                intcode,
                position: 0,
                relative_base: 0,
                input
            }
        }
        pub fn input_mut(&mut self) -> &mut Vec<isize> {
            &mut self.input
        }

        // Helpers
        fn provide_space(&mut self, pos: usize) {
            if pos >= self.intcode.len() {
                self.intcode.resize(pos+1, 0);
            }
        }
        fn get_intcode_mut(&mut self, pos: usize) -> &mut isize {
            self.provide_space(pos);
            &mut self.intcode[pos]
        }
        fn get_intcode_as_pos(&mut self, pos: usize, param_mode: ParameterMode) -> Result<usize,String> {
            match param_mode {
                ParameterMode::POSITION | ParameterMode::RELATIVE => {
                    let mut rel_pos = *self.get_intcode_mut(pos);
                    if param_mode == ParameterMode::RELATIVE {
                        rel_pos += self.relative_base as isize;
                    }
                    Ok(rel_pos as usize)
                }
                ParameterMode::IMMEDIATE => Err(format!("Immediate intcode at {} can't be an pos!", pos))
            }
        }
        fn get_intcode_unpacked(&mut self, pos: usize, param_mode: ParameterMode) -> Result<isize,String> {
            match param_mode {
                ParameterMode::POSITION | ParameterMode::RELATIVE => {
                    let pos = self.get_intcode_as_pos(pos, param_mode)?;
                    self.provide_space(pos);
                    Ok(self.intcode[pos])
                }
                ParameterMode::IMMEDIATE => Ok(*self.get_intcode_mut(pos))
            }
        }

        // Main method
        pub fn process(&mut self) -> Result<IntcodeResult,String> {
            // Iterate through codes
            while let Some(opcode) = self.intcode.get(self.position) {
                let (param_modes, instruction) = (opcode / 100, opcode % 100);
                match instruction {
                    // Add or multiply
                    1 | 2 => {
                        let (param1, param2) = (
                            self.get_intcode_unpacked(self.position+1, ParameterMode::try_from((param_modes, 0))?)?,
                            self.get_intcode_unpacked(self.position+2, ParameterMode::try_from((param_modes, 1))?)?
                        );
                        let intcode_pos = self.get_intcode_as_pos(self.position+3, ParameterMode::try_from((param_modes, 2))?)?;
                        *self.get_intcode_mut(intcode_pos) = match instruction {
                            1 => param1 + param2,
                            2 => param1 * param2,
                            _ => unreachable!()
                        };
                        self.position += 4;
                    }
                    // Input
                    3 => {
                        let intcode_pos = self.get_intcode_as_pos(self.position+1, ParameterMode::try_from((param_modes, 0))?)?;
                        *self.get_intcode_mut(intcode_pos) = *self.input.first().ok_or("Input is missing!".to_string())?;
                        self.input.remove(0);
                        self.position += 2;
                    }
                    // Output
                    4 => {
                        let output = self.get_intcode_unpacked(self.position+1, ParameterMode::try_from((param_modes, 0))?)?;
                        self.position += 2;
                        return Ok(IntcodeResult::OUTPUT(output));
                    }
                    // Jump
                    5 | 6 => {
                        let param1 = self.get_intcode_unpacked(self.position+1, ParameterMode::try_from((param_modes, 0))?)?;
                        if instruction == 5 && param1 != 0 || instruction == 6 && param1 == 0 {
                            self.position = self.get_intcode_unpacked(self.position+2, ParameterMode::try_from((param_modes, 1))?)? as usize;
                        } else {
                            self.position += 3;
                        }
                    }
                    // Less-than | equals
                    7 | 8 => {
                        let (param1, param2) = (
                            self.get_intcode_unpacked(self.position+1, ParameterMode::try_from((param_modes, 0))?)?,
                            self.get_intcode_unpacked(self.position+2, ParameterMode::try_from((param_modes, 1))?)?
                        );
                        let intcode_pos = self.get_intcode_as_pos(self.position+3, ParameterMode::try_from((param_modes, 2))?)?;
                        *self.get_intcode_mut(intcode_pos) =
                            if instruction == 7 && param1 < param2 || instruction == 8 && param1 == param2 {
                                1
                            } else {
                                0
                            };
                        self.position += 4;
                    }
                    // Offset relative base
                    9 => {
                        self.relative_base = (self.relative_base as isize + self.get_intcode_unpacked(self.position+1, ParameterMode::try_from((param_modes, 0))?)?) as usize;
                        self.position += 2;
                    }
                    // Halt!
                    99 => break,
                    // Invalid!
                    _ => return Err(format!("Invalid operation code {} at {}!", opcode, self.position))
                }
            }
            // Halt happened
            Ok(IntcodeResult::HALT)
        }
    }
}
use intcode::*;

// Intcode input
fn read_input_intcode() -> Vec<isize> {
    use std::io::{stdin,BufRead};
    stdin().lock()
        .split(b',')
        .map(|result_token| result_token.expect("Expected token of comma-separated input!"))
        .map(|token| String::from_utf8(token).expect("Input token isn't a valid UTF-8 string!"))
        .map(|string_token| string_token.parse().expect("Input token isn't a signed integer!"))
        .collect()
}

// Parts
fn part1(intcode: &[isize]) -> isize {
    match Processor::new(intcode.to_vec(), vec![1]).process().expect("Intcode processor failed!") {
        IntcodeResult::OUTPUT(out) => out,
        IntcodeResult::HALT => panic!("Intcode processor halted unexpectly!")
    }
}

// Day 9
fn main() {
    let intcode_input = read_input_intcode();
    println!("[Part 1] Keycode: {}", part1(&intcode_input));
}