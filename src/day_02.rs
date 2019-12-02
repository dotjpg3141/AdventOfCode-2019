use std::io::Read;
use std::*;

pub(crate) fn run() -> Result<(), Box<dyn error::Error>> {
    let mut file = fs::File::open("input/day2.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let memory = input
        .split(',')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let result = eval(&memory, 12, 2);
    println!("Day 2a: {}", result);

    let mut result = None;
    'outer: for a in 0..=99 {
        for b in 0..=99 {
            if eval(&memory, a, b) == 19_690_720 {
                result = Some(a * 100 + b);
                break 'outer;
            }
        }
    }
    println!("Day 2b: {:?}", result);

    Ok(())
}

fn eval(memory: &[usize], a: usize, b: usize) -> usize {
    let mut memory = memory.to_vec();
    memory[1] = a;
    memory[2] = b;
    let mut ip = 0;
    loop {
        match memory[ip] {
            1 => {
                let addr = memory[ip + 3];
                memory[addr] = memory[memory[ip + 1]] + memory[memory[ip + 2]];
                ip += 4;
            }
            2 => {
                let addr = memory[ip + 3];
                memory[addr] = memory[memory[ip + 1]] * memory[memory[ip + 2]];
                ip += 4;
            }
            99 => return memory[0],
            opcode => panic!("Invalid opcode={} at ip={}", opcode, ip),
        }
    }
}
