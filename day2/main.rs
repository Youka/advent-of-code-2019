fn main() {
    use std::io::{stdin,BufRead};
    // Read input as integer list
    let mut intcode = stdin().lock()
        .split(b',')
        .map(|result_token| result_token.expect("Expected token of comma-separated input!"))
        .map(|token| String::from_utf8(token).expect("Input token isn't a valid UTF-8 string!"))
        .map(|string_token| string_token.parse().expect("Input token isn't an unsigned integers!"))
        .collect::<Vec<usize>>();
    println!("Intcode input: {:?}", intcode);
    // Restore "1202 program alarm"
    *intcode.get_mut(1).expect("Intcode must have at least 2 elements!") = 12;
    *intcode.get_mut(2).expect("Intcode must have at least 3 elements!") = 2;
    // Go through opcodes + registers
    for i in (0..intcode.len()).step_by(4) {
        match intcode[i] {
            99 => break,
            opcode if (1..=2).contains(&opcode) => {
                let out_reg = intcode[i+3];
                match opcode {
                    1 => intcode[out_reg] = intcode[intcode[i+1]] + intcode[intcode[i+2]],
                    2 => intcode[out_reg] = intcode[intcode[i+1]] * intcode[intcode[i+2]],
                    _ => unreachable!()
                }
            }
            opcode => panic!("Opcode {} at position {} isn't supported!", opcode, i)
        }
    }
    // Output processed intcode
    println!("Intcode output: {:?}", intcode);
}