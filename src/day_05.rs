use std::convert::TryInto;
use std::io::Read;
use std::*;

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let mut file = fs::File::open("input/day5.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let memory = input
        .split(',')
        .map(|s| s.trim().parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let result = eval(&mut memory.clone(), &mut Some(1).into_iter());
    println!("Day 5a: {:?}", result);

    let result = eval(&mut memory.clone(), &mut Some(5).into_iter());
    println!("Day 5b: {:?}", result);

    Ok(())
}

fn eval(memory: &mut [isize], input: &mut impl Iterator<Item = isize>) -> Option<isize> {
    let mut ip = 0;
    let mut output = None;

    fn to_addr(value: isize) -> usize {
        value.try_into().expect("address cannot be negative")
    }

    fn from_bool(b: bool) -> isize {
        if b {
            1
        } else {
            0
        }
    }

    loop {
        let read_parameter = |parameter_index: usize| -> isize {
            let factor = match parameter_index {
                1 => 100,
                2 => 1000,
                3 => 10000,
                other => panic!("invalid parameter index: {}", other),
            };

            let mode = (memory[ip] / factor) % 10;
            match mode {
                0 => {
                    let addr = to_addr(memory[ip + parameter_index]);
                    memory[addr]
                }
                1 => memory[ip + parameter_index],
                other => panic!("invalid parameter mode: {}", other),
            }
        };

        let opcode = memory[ip] % 100;
        match opcode {
            1 => {
                let result_addr = to_addr(memory[ip + 3]);
                memory[result_addr] = read_parameter(1) + read_parameter(2);
                ip += 4;
            }
            2 => {
                let result_addr = to_addr(memory[ip + 3]);
                memory[result_addr] = read_parameter(1) * read_parameter(2);
                ip += 4;
            }
            3 => {
                let result_addr = to_addr(memory[ip + 1]);
                memory[result_addr] = input.next().unwrap();
                ip += 2;
            }
            4 => {
                output = Some(read_parameter(1));
                ip += 2;
            }
            5 => {
                ip = if read_parameter(1) != 0 {
                    to_addr(read_parameter(2))
                } else {
                    ip + 3
                };
            }
            6 => {
                ip = if read_parameter(1) == 0 {
                    to_addr(read_parameter(2))
                } else {
                    ip + 3
                };
            }
            7 => {
                let addr = to_addr(memory[ip + 3]);
                memory[addr] = from_bool(read_parameter(1) < read_parameter(2));
                ip += 4;
            }
            8 => {
                let addr = to_addr(memory[ip + 3]);
                memory[addr] = from_bool(read_parameter(1) == read_parameter(2));
                ip += 4;
            }
            99 => return output,
            opcode => panic!("Invalid opcode={} at ip={}", opcode, ip),
        }
    }
}
