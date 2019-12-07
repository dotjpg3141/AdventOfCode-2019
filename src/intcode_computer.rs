use std::convert::TryInto;
use std::io::Read;
use std::*;

pub(crate) struct IntcodeComputer {
    ip: usize,
    memory: Vec<isize>,
    last_result: IntcodeComputerResult,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum IntcodeComputerRequest {
    Run,
    Input(isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntcodeComputerResult {
    Ready,
    Halt,
    Input,
    Output(isize),
}

impl IntcodeComputer {
    pub fn memory_from_file(path: impl AsRef<path::Path>) -> io::Result<Vec<isize>> {
        let mut file = fs::File::open(path)?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        Ok(Self::parse_memory(&input))
    }

    pub fn parse_memory(input: &str) -> Vec<isize> {
        input
            .split(',')
            .map(|s| s.trim().parse::<isize>().unwrap())
            .collect()
    }

    pub fn with_memory(memory: Vec<isize>) -> Self {
        IntcodeComputer {
            ip: 0,
            memory,
            last_result: IntcodeComputerResult::Ready,
        }
    }

    pub fn execute_step(&mut self, request: IntcodeComputerRequest) -> IntcodeComputerResult {
        match (self.last_result, request) {
            (IntcodeComputerResult::Input, IntcodeComputerRequest::Input(value)) => {
                let result_addr = to_addr(self.memory[self.ip + 1]);
                self.memory[result_addr] = value;
                self.ip += 2;
            }
            (IntcodeComputerResult::Input, _) | (_, IntcodeComputerRequest::Input(_)) => {
                panic!("Expected input, found {:?}/{:?}", self.last_result, request);
            }
            (IntcodeComputerResult::Halt, _) => {
                panic!("Cannot run halted computer");
            }
            _ => {}
        }

        loop {
            let read_parameter = |parameter_index: usize| -> isize {
                let factor = match parameter_index {
                    1 => 100,
                    2 => 1000,
                    3 => 10000,
                    other => panic!("invalid parameter index: {}", other),
                };

                let mode = (self.memory[self.ip] / factor) % 10;
                match mode {
                    0 => {
                        let addr = to_addr(self.memory[self.ip + parameter_index]);
                        self.memory[addr]
                    }
                    1 => self.memory[self.ip + parameter_index],
                    other => panic!("invalid parameter mode: {}", other),
                }
            };

            let opcode = self.memory[self.ip] % 100;
            match opcode {
                1 => {
                    let result_addr = to_addr(self.memory[self.ip + 3]);
                    self.memory[result_addr] = read_parameter(1) + read_parameter(2);
                    self.ip += 4;
                }
                2 => {
                    let result_addr = to_addr(self.memory[self.ip + 3]);
                    self.memory[result_addr] = read_parameter(1) * read_parameter(2);
                    self.ip += 4;
                }
                3 => {
                    self.last_result = IntcodeComputerResult::Input;
                    break;
                }
                4 => {
                    self.last_result = IntcodeComputerResult::Output(read_parameter(1));
                    self.ip += 2;
                    break;
                }
                5 => {
                    self.ip = if read_parameter(1) != 0 {
                        to_addr(read_parameter(2))
                    } else {
                        self.ip + 3
                    };
                }
                6 => {
                    self.ip = if read_parameter(1) == 0 {
                        to_addr(read_parameter(2))
                    } else {
                        self.ip + 3
                    };
                }
                7 => {
                    let addr = to_addr(self.memory[self.ip + 3]);
                    self.memory[addr] = from_bool(read_parameter(1) < read_parameter(2));
                    self.ip += 4;
                }
                8 => {
                    let addr = to_addr(self.memory[self.ip + 3]);
                    self.memory[addr] = from_bool(read_parameter(1) == read_parameter(2));
                    self.ip += 4;
                }
                99 => {
                    self.last_result = IntcodeComputerResult::Halt;
                    break;
                }
                opcode => panic!("Invalid opcode={} at ip={}", opcode, self.ip),
            }
        }

        self.last_result
    }

    pub fn eval(&mut self, input: &mut impl Iterator<Item = isize>) -> Vec<isize> {
        let mut output = Vec::new();
        let mut request = IntcodeComputerRequest::Run;

        loop {
            let result = self.execute_step(request);
            match result {
                IntcodeComputerResult::Halt => return output,
                IntcodeComputerResult::Input => {
                    let input = input.next().expect("End of input reached");
                    request = IntcodeComputerRequest::Input(input);
                }
                IntcodeComputerResult::Output(value) => {
                    output.push(value);
                    request = IntcodeComputerRequest::Run;
                }
                IntcodeComputerResult::Ready => unreachable!(),
            }
        }
    }

    pub fn read(&mut self) -> Result<isize, ()> {
        loop {
            match self.last_result {
                IntcodeComputerResult::Output(value) => return Ok(value),
                IntcodeComputerResult::Ready => {}
                IntcodeComputerResult::Halt => return Err(()),
                other => panic!("Unexpected read state: {:?}", other),
            }

            self.execute_step(IntcodeComputerRequest::Run);
        }
    }

    pub fn write(&mut self, value: isize) -> Result<(), ()> {
        loop {
            match self.last_result {
                IntcodeComputerResult::Input => {
                    self.execute_step(IntcodeComputerRequest::Input(value));
                    return Ok(());
                }
                IntcodeComputerResult::Halt => return Err(()),
                _ => {}
            }

            self.execute_step(IntcodeComputerRequest::Run);
        }
    }
}

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
