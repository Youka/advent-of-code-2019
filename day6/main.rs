use std::str::FromStr;

// Orbit
#[derive(Debug,Eq,PartialEq)]
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
            sum += count_indirect_and_direct_orbits(orbits, &orbit, depth+1);
        }
        sum
    })
}
fn find_lowest_orbits_to_santa(orbits: &[Orbit], current: &Orbit, previous: Option<&Orbit>, depth: usize) -> Option<usize> {
    if current.border_object == "SAN" {
        Some(depth)
    } else {
        orbits.iter().fold(None, |mut found, orbit| {
            if Some(orbit) != previous && (current.border_object == orbit.center_object || current.center_object == orbit.border_object) {
                if let Some(possible_found) = find_lowest_orbits_to_santa(orbits, &orbit, Some(current), depth+1) {
                    found = found.map(|f| f.min(possible_found)).or(Some(possible_found));
                }
            }
            found
        })
    }
}

// Parts
fn part1(orbits: &[Orbit]) -> usize {
    count_indirect_and_direct_orbits(orbits, orbits.iter().find(|orbit| orbit.center_object == "COM" ).expect("No center-of-mass?"), 0)
}
fn part2(orbits: &[Orbit]) -> Option<usize> {
    find_lowest_orbits_to_santa(orbits, orbits.iter().find(|orbit| orbit.border_object == "YOU" ).expect("No own position?"), None, 0).map(|way| way-2 )
}

// Day 6
fn main() {
    let orbits = read_input_orbits();
    println!("[Part 1] Checksum: {}", part1(&orbits));
    println!("[Part 2] Orbital transfers: {:?}", part2(&orbits));
}