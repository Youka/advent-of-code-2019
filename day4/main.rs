// Imports
use std::{
    collections::HashMap,
    io::{stdin,Read},
    ops::RangeInclusive
};

// Input
fn read_input_range() -> Option<RangeInclusive<u32>> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).ok()?;
    let separator = input.find('-')?;
    Some( input[..separator].parse::<u32>().ok()? ..= input[separator+1..].parse::<u32>().ok()? )
}

// Checks
fn has_six_digits(num: u32) -> bool {
    (100_000..=999_999).contains(&num)
}
fn has_2_adjacent_digits(digits: &[u8]) -> bool {
    digits.first().and_then(|first_digit|
        digits.iter()
            .skip(1)
            .try_fold(first_digit, |last_digit, digit| if digit == last_digit {None} else {Some(digit)} )
            .map_or(Some(()), |_| None )
    ).is_some()
}
fn has_increasing_digits(digits: &[u8]) -> bool {
    digits.first().and_then(|first_digit|
        digits.iter()
            .skip(1)
            .try_fold(first_digit, |last_digit, digit| if digit >= last_digit {Some(digit)} else {None} )
    ).is_some()
}
fn has_digit_twice(digits: &[u8]) -> bool {
    let mut map = HashMap::with_capacity(2);
    digits.iter().for_each(|digit| *map.entry(digit).or_insert(0) += 1 );
    map.iter().any(|(_,digit_number)| *digit_number == 2 )
}

// Day 4
fn main() {
    let hits = read_input_range().expect("Input incorrect!")
        .filter(|num| has_six_digits(*num) )
        .map(|num| num.to_string().into_bytes() )
        .filter(|digits| has_2_adjacent_digits(digits) && has_increasing_digits(digits) )
        .collect::<Vec<_>>();
    println!("Part 1: {}", hits.len());
    println!("Part 2: {}", hits.iter().filter(|digits| has_digit_twice(digits) ).count());
}