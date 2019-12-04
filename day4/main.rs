// Imports
use std::{
    io::{stdin,Read},
    ops::RangeInclusive
};

// Input
fn read_input_range() -> RangeInclusive<u32> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).expect("No input?!");
    if let Some(separator) = input.find('-') {
        input[..separator].parse::<u32>().expect("Range start isn't a number!") ..=
        input[separator+1..].parse::<u32>().expect("Range end isn't a number!")
    } else {
        panic!("Range separator not found!");
    }
}

// Checks
fn is_six_digit(num: u32) -> bool {
    num < 1_000_000
}
fn has_2_adjacent_digits(num: u32) -> bool {
    let chars = num.to_string().chars().collect::<Vec<char>>();
    chars.first().and_then(|first_char|
        chars.iter()
            .skip(1)
            .try_fold(first_char, |last_char, character| if character == last_char {None} else {Some(character)} )
            .map_or(Some(()), |_| None )
    ).is_some()
}
fn has_increasing_digits(num: u32) -> bool {
    let chars = num.to_string().chars().collect::<Vec<char>>();
    chars.first().and_then(|first_char|
        chars.iter()
            .skip(1)
            .try_fold(first_char, |last_char, character| if character >= last_char {Some(character)} else {None} )
    ).is_some()
}

// Day 4
fn main() {
    println!(
        "Hits: {}",
        read_input_range().filter(|num| {
            is_six_digit(*num) && has_2_adjacent_digits(*num) && has_increasing_digits(*num)
        }).count()
    );
}