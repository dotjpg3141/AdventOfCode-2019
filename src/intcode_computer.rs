use std::convert::TryInto;
use std::io::Read;
use std::*;

type Address = usize;
type Value = i64;

pub(crate) struct IntcodeComputer {
    ip: Address,
    memory: Vec<Value>,
    relative_base: Value,
    last_result: IntcodeComputerResult,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum IntcodeComputerRequest {
    Run,
    Input(Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntcodeComputerResult {
    Ready,
    Halt,
    Input,
    Output(Value),
}

impl IntcodeComputer {
    pub fn memory_from_file(path: impl AsRef<path::Path>) -> io::Result<Vec<Value>> {
        let mut file = fs::File::open(path)?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        Ok(Self::parse_memory(&input))
    }

    pub fn parse_memory(input: &str) -> Vec<Value> {
        input
            .split(',')
            .map(|s| s.trim().parse::<Value>().unwrap())
            .collect()
    }

    pub fn with_memory(memory: Vec<Value>) -> Self {
        IntcodeComputer {
            ip: 0,
            memory,
            relative_base: 0,
            last_result: IntcodeComputerResult::Ready,
        }
    }

    pub fn execute_step(&mut self, request: IntcodeComputerRequest) -> IntcodeComputerResult {
        match (self.last_result, request) {
            (IntcodeComputerResult::Input, IntcodeComputerRequest::Input(value)) => {
                let result_addr = self.read_addr(1);
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
            let opcode = self.memory[self.ip] % 100;
            match opcode {
                1 => {
                    let addr = self.read_addr(3);
                    self.memory[addr] = self.read_value(1) + self.read_value(2);
                    self.ip += 4;
                }
                2 => {
                    let addr = self.read_addr(3);
                    self.memory[addr] = self.read_value(1) * self.read_value(2);
                    self.ip += 4;
                }
                3 => {
                    self.last_result = IntcodeComputerResult::Input;
                    break;
                }
                4 => {
                    self.last_result = IntcodeComputerResult::Output(self.read_value(1));
                    self.ip += 2;
                    break;
                }
                5 => {
                    self.ip = if self.read_value(1) != 0 {
                        to_addr(self.read_value(2))
                    } else {
                        self.ip + 3
                    };
                }
                6 => {
                    self.ip = if self.read_value(1) == 0 {
                        to_addr(self.read_value(2))
                    } else {
                        self.ip + 3
                    };
                }
                7 => {
                    let addr = self.read_addr(3);
                    self.memory[addr] = from_bool(self.read_value(1) < self.read_value(2));
                    self.ip += 4;
                }
                8 => {
                    let addr = self.read_addr(3);
                    self.memory[addr] = from_bool(self.read_value(1) == self.read_value(2));
                    self.ip += 4;
                }
                9 => {
                    self.relative_base += self.read_value(1);
                    self.ip += 2;
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

    fn read_value(&self, parameter_index: Address) -> Value {
        let addr = self.read_addr(parameter_index);
        self.memory[addr]
    }

    fn read_addr(&self, parameter_index: Address) -> Address {
        let factor = match parameter_index {
            1 => 100,
            2 => 1000,
            3 => 10000,
            other => panic!("invalid parameter index: {}", other),
        };

        let mode = (self.memory[self.ip] / factor) % 10;
        let addr = self.ip + parameter_index;
        match mode {
            0 => to_addr(self.memory[addr]),
            1 => addr,
            2 => to_addr(self.memory[addr] + self.relative_base),
            other => panic!("invalid parameter mode: {}", other),
        }
    }

    pub fn eval(&mut self, input: &mut impl Iterator<Item = Value>) -> Vec<Value> {
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

    pub fn read(&mut self) -> Result<Value, ()> {
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

    pub fn write(&mut self, value: Value) -> Result<(), ()> {
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

fn to_addr(value: Value) -> Address {
    value.try_into().expect("cannot convert value to address")
}

fn from_bool(b: bool) -> Value {
    if b {
        1
    } else {
        0
    }
}
