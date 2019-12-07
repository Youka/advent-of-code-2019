use std::str::FromStr;

// Orbit
struct Orbit {
    pub center_object: String,
    pub border_object: String
}
impl FromStr for Orbit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let separator = s.find(')').ok_or(format!("Invalid orbit format: {}", s))?;
        Ok(Self {
            center_object: s[..separator].to_string(),
            border_object: s[separator+1..].to_string()
        })
    }
}

// Input
fn read_input_orbits() -> Vec<Orbit> {
    use std::io::{stdin,BufRead};
    stdin().lock()
        .lines()
        .map(|result_line| result_line.expect("Line from input expected!") )
        .map(|line| Orbit::from_str(&line).expect("Input invalid!") )
        .collect()
}

// Traverse orbits
fn count_indirect_and_direct_orbits(orbits: &[Orbit], current: &Orbit, depth: usize) -> usize {
    orbits.iter().fold(depth+1, |mut sum, orbit| {
        if current.border_object == orbit.center_object {
            sum += count_indirect_and_direct_orbits(orbits, &orbit, depth+1)
        }
        sum
    })
}

// Parts
fn part1(orbits: &[Orbit]) -> usize {
    count_indirect_and_direct_orbits(orbits, orbits.iter().find(|orbit| orbit.center_object == "COM" ).expect("No center-of-mass?"), 0)
}

// Day 6
fn main() {
    let orbits = read_input_orbits();
    println!("[Part 1] Checksum: {}", part1(&orbits));
}