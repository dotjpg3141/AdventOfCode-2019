use itertools::Itertools;
use std::*;

use crate::intcode_computer::*;

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let memory = IntcodeComputer::memory_from_file("input/day07.txt")?;

    let result = (0..=4)
        .permutations(5)
        .map(|phases| {
            let mut value = 0;
            for phase in phases {
                let mut computer = IntcodeComputer::with_memory(memory.clone());
                let result = computer.eval(&mut vec![phase, value].into_iter());
                value = *result.last().expect("Expected output");
            }
            value
        })
        .max();
    println!("Day 7a: {:?}", result);

    let result = (5..=9)
        .permutations(5)
        .map(|phases| {
            let mut computers = phases
                .into_iter()
                .map(|phase| {
                    let mut computer = IntcodeComputer::with_memory(memory.clone());
                    computer.write(phase).expect("Unexpected end of computer");
                    computer
                })
                .collect::<Vec<_>>();

            let mut value = 0;

            (0..5)
                .cycle()
                .map(|index| -> Result<(), ()> {
                    let computer = &mut computers[index];
                    computer.write(value)?;
                    value = computer.read()?;
                    Ok(())
                })
                .take_while(Result::is_ok)
                .for_each(drop);

            value
        })
        .max();
    println!("Day 7b: {:?}", result);

    Ok(())
}
