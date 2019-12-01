fn module_fuel_required(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

fn module_fuel_required_recursive(mass: usize) -> usize {
    let fuel = module_fuel_required(mass);
    if fuel == 0 {0} else {fuel + module_fuel_required_recursive(fuel)}
}

fn main() {
    use std::io::{stdin, BufRead};
    println!(
        "Summarized fuel required: {:?}",
        stdin().lock()
            .lines()
            .enumerate()
            .fold((0, 0), |(sum, sum_recursive), (i, line)| {
                let mass = line.expect("Text line expected!").parse().expect(&format!("Line {} isn't an unsigned integer!", 1+i));
                let fuel = module_fuel_required(mass);
                let fuel_recursive = module_fuel_required_recursive(mass);
                println!("{:3}: Mass {:7} requires ({:6},{:6}) fuel", 1+i, mass, fuel, fuel_recursive);
                (sum + fuel, sum_recursive + fuel_recursive)
            })
    );
}