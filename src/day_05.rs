use std::*;

use crate::intcode_computer::*;

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let memory = IntcodeComputer::memory_from_file("input/day05.txt")?;

    let result = eval(&memory, 1);
    println!("Day 5a: {:?}", result);

    let result = eval(&memory, 5);
    println!("Day 5b: {:?}", result);

    Ok(())
}

fn eval(memory: &[i64], input: i64) -> Option<i64> {
    let mut computer = IntcodeComputer::with_memory(memory.to_owned());
    computer.eval(&mut Some(input).into_iter()).last().cloned()
}
