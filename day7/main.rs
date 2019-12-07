// Intcode processig module
mod intcode {
    // Imports
    use std::convert::TryFrom;

    // Private (helpers)
    enum ParameterMode {
        POSITION,
        IMMEDIATE
    }
    impl TryFrom<(isize, u8)> for ParameterMode {
        type Error = String;
        fn try_from(modes_pos: (isize, u8)) -> Result<Self, Self::Error> {
            match modes_pos.0 / (10_isize.pow(modes_pos.1 as u32)) % 10 {
                0 => Ok(Self::POSITION),
                1 => Ok(Self::IMMEDIATE),
                _ => Err( format!("Invalid parameter mode at {} in {}!", modes_pos.1, modes_pos.0) )
            }
        }
    }

    fn get_intcode(intcode: &[isize], pos: usize) -> Result<isize,String> {
        intcode.get(pos).map(|code| *code).ok_or(format!("Parameter at position {} missing!", pos))
    }
    fn get_intcode_as_index(intcode: &[isize], pos: usize) -> Result<usize,String> {
        usize::try_from( get_intcode(intcode, pos)? ).map_err(|_| format!("Parameter at {} not an unsigned integer!", pos) )
    }
    fn get_intcode_parametrized(intcode: &[isize], pos: usize, param_mode: ParameterMode) -> Result<isize,String> {
        match param_mode {
            ParameterMode::POSITION => Ok(intcode[get_intcode_as_index(intcode, pos)?]),
            ParameterMode::IMMEDIATE => Ok(get_intcode(intcode, pos)?)
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
        input: Vec<isize>
    }
    impl Processor {
        pub fn new(intcode: Vec<isize>, input: Vec<isize>) -> Self {
            Self {
                intcode,
                position: 0,
                input
            }
        }
        pub fn input_mut(&mut self) -> &mut Vec<isize> {
            &mut self.input
        }
        pub fn process(&mut self) -> Result<IntcodeResult,String> {
            // Iterate through codes
            while let Some(opcode) = self.intcode.get(self.position) {
                let (param_modes, instruction) = (opcode / 100, opcode % 100);
                match instruction {
                    // Add or multiply
                    1 | 2 => {
                        let (param1, param2) = (
                            get_intcode_parametrized(&self.intcode, self.position+1, ParameterMode::try_from((param_modes, 0))?)?,
                            get_intcode_parametrized(&self.intcode, self.position+2, ParameterMode::try_from((param_modes, 1))?)?
                        );
                        let intcode_index = get_intcode_as_index(&self.intcode, self.position+3)?;
                        self.intcode[intcode_index] = match instruction {
                            1 => param1 + param2,
                            2 => param1 * param2,
                            _ => unreachable!()
                        };
                        self.position += 4;
                    }
                    // Input
                    3 => {
                        let intcode_index = get_intcode_as_index(&self.intcode, self.position+1)?;
                        self.intcode[intcode_index] = *self.input.first().ok_or("Input is missing!".to_string())?;
                        self.input.remove(0);
                        self.position += 2;
                    }
                    // Output
                    4 => {
                        let output = get_intcode_parametrized(&self.intcode, self.position+1, ParameterMode::try_from((param_modes, 0))?)?;
                        self.position += 2;
                        return Ok(IntcodeResult::OUTPUT(output));
                    }
                    // Jump
                    5 | 6 => {
                        let param1 = get_intcode_parametrized(&self.intcode, self.position+1, ParameterMode::try_from((param_modes, 0))?)?;
                        if instruction == 5 && param1 != 0 || instruction == 6 && param1 == 0 {
                            self.position = get_intcode_parametrized(&self.intcode, self.position+2, ParameterMode::try_from((param_modes, 1))?)? as usize;
                        } else {
                            self.position += 3;
                        }
                    }
                    // Less-than | equals
                    7 | 8 => {
                        let (param1, param2) = (
                            get_intcode_parametrized(&self.intcode, self.position+1, ParameterMode::try_from((param_modes, 0))?)?,
                            get_intcode_parametrized(&self.intcode, self.position+2, ParameterMode::try_from((param_modes, 1))?)?
                        );
                        let intcode_index = get_intcode_as_index(&self.intcode, self.position+3)?;
                        self.intcode[intcode_index] =
                            if instruction == 7 && param1 < param2 || instruction == 8 && param1 == param2 {
                                1
                            } else {
                                0
                            };
                        self.position += 4;
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

// Permutation module
mod permutation {
    pub fn permutations_recursive<T>(items: &mut [T], n: usize, results: &mut Vec<Vec<T>>) where T: Clone {
        match n {
            0 => results.push( items.iter().cloned().collect() ),
            _ => for i in 0..n {
                items.swap(i, n-1);
                permutations_recursive(items, n-1, results);
                items.swap(i, n-1);
            }
        }
    }
    pub fn all_permutations<T>(mut items: Vec<T>) -> Vec<Vec<T>> where T: Clone {
        let n = items.len();
        let mut results = Vec::with_capacity( (1..=n).product() );
        permutations_recursive(&mut items, n, &mut results);
        results
    }
}
use permutation::*;

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
fn part1(intcode: Vec<isize>) -> Option<isize> {
    // Try all phase settings
    all_permutations( (0..=4).collect() ).into_iter().map(|phase_settings| {
        // Process amplifiers
        phase_settings.into_iter().fold(0, |in_out, phase_setting| {
            match Processor::new(intcode.clone(), vec![phase_setting, in_out]).process().expect("Intcode processing error occured!") {
                IntcodeResult::OUTPUT(out) => out,
                IntcodeResult::HALT => panic!("Halting too soon!")
            }
        })
    }).max()
}
fn part2(intcode: Vec<isize>) -> Option<isize> {
    // Try all phase settings
    all_permutations( (5..=9).collect() ).into_iter().map(|phase_settings| {
        // Create amplifiers
        let mut processors = phase_settings.into_iter().map(|phase_setting|
            Processor::new(intcode.clone(), vec![phase_setting])
        ).collect::<Vec<_>>();
        // Feedback loop amplifiers and detect max. output
        let mut in_out = 0;
        for processors_index in (0..processors.len()).cycle() {
            let processor = &mut processors[processors_index];
            processor.input_mut().push(in_out);
            match processor.process().expect("Processor error occured!") {
                IntcodeResult::OUTPUT(out) => in_out = out,
                IntcodeResult::HALT => break
            }
        }
        in_out
    }).max()
}

// Day 7
fn main() {
    let intcode_input = read_input_intcode();
    println!("[Part 1] Max output signal: {:?}", part1(intcode_input.clone()));
    println!("[Part 2] Max output signal: {:?}", part2(intcode_input));
}