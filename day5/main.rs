// Parameter modes
enum ParameterMode {
    POSITION,
    IMMEDIATE
}
impl From<(isize, u8)> for ParameterMode {
    fn from(modes_pos: (isize, u8)) -> Self {
        match modes_pos.0 / (10_isize.pow(modes_pos.1 as u32)) % 10 {
            0 => Self::POSITION,
            1 => Self::IMMEDIATE,
            _ => panic!("Invalid parameter mode at {} in {}!", modes_pos.1, modes_pos.0)
        }
    }
}

// Converter
fn get_intcode(intcode: &[isize], pos: usize) -> isize {
    *intcode.get(pos).expect(&format!("Parameter at position {} missing!", pos))
}
fn intcode_to_index(intcode: &[isize], pos: usize) -> usize {
    use std::convert::TryFrom;
    usize::try_from(get_intcode(intcode, pos)).expect(&format!("Parameter at {} not an unsigned integer!", pos))
}
fn get_intcode_parametrized(intcode: &[isize], pos: usize, param_mode: ParameterMode) -> isize {
    match param_mode {
        ParameterMode::POSITION => intcode[intcode_to_index(intcode, pos)],
        ParameterMode::IMMEDIATE => get_intcode(intcode, pos)
    }
}

// Intcode processing
fn process_intcode(intcode: &mut [isize], input: isize) -> Vec<isize> {
    // Output buffer
    let mut output = vec![];
    // Iterate through codes
    let mut pos = 0;
    while let Some(opcode) = intcode.get(pos) {
        let (param_modes, instruction) = (opcode / 100, opcode % 100);
        match instruction {
            // Add or multiply
            1 | 2 => {
                let (param1, param2) = (
                    get_intcode_parametrized(intcode, pos+1, ParameterMode::from((param_modes, 0))),
                    get_intcode_parametrized(intcode, pos+2, ParameterMode::from((param_modes, 1)))
                );
                intcode[intcode_to_index(intcode, pos+3)] = match instruction {
                    1 => param1 + param2,
                    2 => param1 * param2,
                    _ => unreachable!()
                };
                pos += 4;
            }
            // Input
            3 => {
                intcode[intcode_to_index(intcode, pos+1)] = input;
                pos += 2;
            }
            // Output
            4 => {
                output.push(get_intcode_parametrized(intcode, pos+1, ParameterMode::from((param_modes, 0))));
                pos += 2;
            }
            // Jump
            5 | 6 => {
                let param1 = get_intcode_parametrized(intcode, pos+1, ParameterMode::from((param_modes, 0)));
                if instruction == 5 && param1 != 0 || instruction == 6 && param1 == 0 {
                    pos = get_intcode_parametrized(intcode, pos+2, ParameterMode::from((param_modes, 1))) as usize;
                } else {
                    pos += 3;
                }
            }
            // Less-than | equals
            7 | 8 => {
                let (param1, param2) = (
                    get_intcode_parametrized(intcode, pos+1, ParameterMode::from((param_modes, 0))),
                    get_intcode_parametrized(intcode, pos+2, ParameterMode::from((param_modes, 1)))
                );
                intcode[intcode_to_index(intcode, pos+3)] =
                    if instruction == 7 && param1 < param2 || instruction == 8 && param1 == param2 {
                        1
                    } else {
                        0
                    };
                pos += 4;
            }
            // Halt!
            99 => break,
            // Invalid!
            _ => panic!("Invalid operation code: {}", opcode)
        }
    }
    // Return output, ending with diagnostic code
    output
}

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

// Day 5
fn main() {
    let mut intcode = read_input_intcode();
    println!("[Part 1] Intcode output: {:?}", process_intcode(&mut intcode.clone(), 1));
    println!("[Part 2] Intcode output: {:?}", process_intcode(&mut intcode, 5));
}