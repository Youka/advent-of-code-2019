fn process_intcode(mut intcode: Vec<usize>, noun: usize, verb: usize) -> usize {
    // Insert "noun" and "verb"
    *intcode.get_mut(1).expect("Intcode must have at least 2 elements!") = noun;
    *intcode.get_mut(2).expect("Intcode must have at least 3 elements!") = verb;
    // Go through opcodes + registers
    for pos in (0..intcode.len()).step_by(4) {
        match intcode[pos] {
            // Halt program
            99 => break,
            // Arithmetic operations
            opcode if (1..=2).contains(&opcode) => {
                let out_addr = intcode[pos+3];
                intcode[out_addr] = match opcode {
                    1 => intcode[intcode[pos+1]] + intcode[intcode[pos+2]],
                    2 => intcode[intcode[pos+1]] * intcode[intcode[pos+2]],
                    _ => unreachable!()
                }
            }
            // Error!
            opcode => panic!("Opcode {} at position {} isn't supported!", opcode, pos)
        }
    }
    *intcode.first().expect("Intcode shouldn't be empty!")
}

fn puzzle_part1(intcode: &Vec<usize>) {
    println!("[Part 1] Output: {}", process_intcode(intcode.clone(), 12, 2));
}

fn puzzle_part2(intcode: &Vec<usize>) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if process_intcode(intcode.clone(), noun, verb) == 19690720_usize {
                println!("[Part 2] Noun={} - Verb={} - Result={}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}

fn main() {
    use std::io::{stdin,BufRead};
    // Read input as integer list
    let intcode = stdin().lock()
        .split(b',')
        .map(|result_token| result_token.expect("Expected token of comma-separated input!"))
        .map(|token| String::from_utf8(token).expect("Input token isn't a valid UTF-8 string!"))
        .map(|string_token| string_token.parse().expect("Input token isn't an unsigned integer!"))
        .collect::<Vec<usize>>();
    // Solve puzzles
    puzzle_part1(&intcode);
    puzzle_part2(&intcode);
}