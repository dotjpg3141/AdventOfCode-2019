use std::io::BufRead;
use std::*;

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open("input/day01.txt")?;
    let lines = io::BufReader::new(file).lines();

    let fuel_input = lines
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let total_fuel: isize = fuel_input.iter().cloned().map(calculate_fuel).sum();
    println!("Day 1a: {}", total_fuel);

    let total_fuel: isize = fuel_input.iter().cloned().map(calculate_total_fuel).sum();
    println!("Day 1b: {}", total_fuel);

    Ok(())
}

fn calculate_fuel(mass: isize) -> isize {
    cmp::max(mass / 3 - 2, 0)
}

fn calculate_total_fuel(mass: isize) -> isize {
    let mut remaining_mass = mass;
    let mut total_fuel = 0;

    while remaining_mass > 0 {
        let additional_fuel = calculate_fuel(remaining_mass);
        total_fuel += additional_fuel;
        remaining_mass = additional_fuel;
    }

    total_fuel
}
