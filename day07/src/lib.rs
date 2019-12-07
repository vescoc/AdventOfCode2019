#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

const SEPARATOR: char = ',';

#[derive(Debug)]
pub enum OpcodeMode {
    Positional,
    Immediate,
}

pub type Opcode = u64;
pub type Memory = i64;

#[derive(Debug)]
pub enum Error {
    InvalidOpcode(Opcode, usize),
    InvalidOpcodeMode(Opcode, OpcodeMode),
    InvalidOpcodeModeIndex(Opcode, Opcode),
    InvalidOpcodeModeValue(Opcode, Opcode),
    EOF,
    NoInput(usize),
}

#[derive(Debug)]
pub enum Step {
    Continue,
    NeedInput,
    Output(Memory),
    Halt,
}

fn parse(data: &str) -> Vec<Memory> {
    data.trim()
        .split(SEPARATOR)
        .map(|s| {
            s.parse()
                .unwrap_or_else(|e| panic!("cannot parse: {}, {}", s, e))
        })
        .collect()
}

lazy_static! {
    static ref DATA: Vec<Memory> = parse(include_str!("../data.txt"));
}

pub struct CPU {
    memory: Vec<Memory>,
    ip: usize,
    input: Option<Memory>,
}

impl CPU {
    pub fn new(memory: Vec<Memory>, ip: usize, input: Option<Memory>) -> Self {
        Self { memory, ip, input }
    }

    pub fn step(&mut self) -> Result<Step, Error> {
        let memory = self.memory.as_mut_slice();

        let opcode = memory.get(self.ip).ok_or_else(|| Error::EOF)?.to_owned() as Opcode;

        let opcode_mode = |i| match i {
            0 => Ok(OpcodeMode::Positional),
            1 => Ok(OpcodeMode::Immediate),
            _ => Err(Error::InvalidOpcodeModeValue(opcode, i)),
        };

        let mode = |i| match i {
            1 => opcode_mode(opcode / 100 % 10),
            2 => opcode_mode(opcode / 1000 % 10),
            3 => opcode_mode(opcode / 10000 % 10),
            _ => Err(Error::InvalidOpcodeModeIndex(opcode, i)),
        };

        let write = |is: &mut [Memory], i, m, v| match m {
            OpcodeMode::Positional => {
                is[is[i] as usize] = v;
                Ok(())
            }
            OpcodeMode::Immediate => Err(Error::InvalidOpcodeMode(opcode, m)),
        };

        let read = |is: &[Memory], i, m| -> Result<Memory, Error> {
            match m {
                OpcodeMode::Positional => Ok(is[is[i] as usize]),
                OpcodeMode::Immediate => Ok(is[i] as Memory),
            }
        };

        match opcode % 100 {
            1 => {
                write(
                    memory,
                    self.ip + 3,
                    mode(3)?,
                    read(memory, self.ip + 1, mode(1)?)? + read(memory, self.ip + 2, mode(2)?)?,
                )?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            2 => {
                write(
                    memory,
                    self.ip + 3,
                    mode(3)?,
                    read(memory, self.ip + 1, mode(1)?)? * read(memory, self.ip + 2, mode(2)?)?,
                )?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            3 => {
                if let Some(input) = self.input {
                    write(memory, self.ip + 1, OpcodeMode::Positional, input)?;
                    self.input = None;
                    self.ip += 2;

                    Ok(Step::Continue)
                } else {
                    Ok(Step::NeedInput)
                }
            }
            4 => {
                let output = read(memory, self.ip + 1, mode(1)?)?;
                self.ip += 2;

                Ok(Step::Output(output))
            }
            5 => {
                self.ip = if read(memory, self.ip + 1, mode(1)?)? != 0 {
                    read(memory, self.ip + 2, mode(2)?)? as usize
                } else {
                    self.ip + 3
                };

                Ok(Step::Continue)
            }
            6 => {
                self.ip = if read(memory, self.ip + 1, mode(1)?)? == 0 {
                    read(memory, self.ip + 2, mode(2)?)? as usize
                } else {
                    self.ip + 3
                };

                Ok(Step::Continue)
            }
            7 => {
                let value = if read(memory, self.ip + 1, mode(1)?)?
                    < read(memory, self.ip + 2, mode(2)?)?
                {
                    1
                } else {
                    0
                };

                write(memory, self.ip + 3, mode(3)?, value)?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            8 => {
                let value = if read(memory, self.ip + 1, mode(1)?)?
                    == read(memory, self.ip + 2, mode(2)?)?
                {
                    1
                } else {
                    0
                };

                write(memory, self.ip + 3, mode(3)?, value)?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            99 => Ok(Step::Halt),

            _ => Err(Error::InvalidOpcode(opcode, self.ip)),
        }
    }

    pub fn fork(&self, input: Option<Memory>) -> Self {
        Self {
            memory: self.memory.to_owned(),
            ip: self.ip,
            input,
        }
    }

    pub fn run(&mut self) -> Result<Step, Error> {
        loop {
            match self.step() {
                Ok(Step::Continue) => {}
                r => break r,
            }
        }
    }
}

pub fn solve_1(base_memory: &[Memory]) -> (Memory, Vec<usize>) {
    let mut base_cpu = CPU::new(base_memory.to_owned(), 0, None);
    match base_cpu.run() {
        Ok(Step::NeedInput) => {}
        s => panic!("invalid state {:?}", s),
    }

    let cpus = (0usize..5usize)
        .try_fold(HashMap::new(), |mut acc, i| {
            let mut cpu = base_cpu.fork(Some(i as Memory));

            match cpu.run() {
                Ok(Step::NeedInput) => {
                    acc.insert(i, cpu);
                    Ok(acc)
                }
                Err(e) => Err(e),
                _ => unreachable!(),
            }
        })
        .unwrap_or_else(|e| panic!("invalid data: {:?}", e));

    (0usize..5usize)
        .permutations(5)
        .try_fold(
            (std::i64::MIN, vec![]),
            |(current_max, current_permutation), p| {
                p.iter()
                    .try_fold(0, |acc, i| {
                        let mut cpu = cpus[i].fork(Some(acc));

                        match cpu.run() {
                            Ok(Step::Output(value)) => Ok(value),
                            Err(e) => Err(e),
                            _ => unreachable!(),
                        }
                    })
                    .and_then(|value| match value.cmp(&current_max) {
                        Ordering::Greater => Ok((value, p)),
                        _ => Ok((current_max, current_permutation)),
                    })
            },
        )
        .unwrap_or_else(|e| panic!("invalid state: {:?}", e))
}

pub fn solve_2(base_memory: &[Memory]) -> (Memory, Vec<usize>) {
    let mut base_cpu = CPU::new(base_memory.to_owned(), 0, None);
    match base_cpu.run() {
        Ok(Step::NeedInput) => {}
        s => panic!("invalid state {:?}", s),
    }

    let cpus = (5usize..10usize)
        .try_fold(HashMap::new(), |mut acc, i| {
            let mut cpu = base_cpu.fork(Some(i as Memory));

            match cpu.run() {
                Ok(Step::NeedInput) => {
                    acc.insert(i, cpu);
                    Ok(acc)
                }
                Err(e) => Err(e),
                _ => unreachable!(),
            }
        })
        .unwrap_or_else(|e| panic!("invalid data: {:?}", e));

    (5usize..10usize)
        .permutations(5)
        .try_fold(
            (std::i64::MIN, vec![]),
            |(current_max, current_permutation), p| {
                let mut cpus = cpus
                    .iter()
                    .map(|(k, v)| (k, v.fork(v.input)))
                    .collect::<HashMap<_, _>>();

                let p_last = p.last().unwrap();

                p.iter()
                    .cycle()
                    .try_fold((Some(0), None), |(input, output), i| {
                        let cpu = cpus.get_mut(i).unwrap();
                        cpu.input = input;

                        match cpu.run() {
                            Ok(Step::Output(value)) if i == p_last => {
                                Ok((Some(value), Some(value)))
                            }
                            Ok(Step::Output(value)) => Ok((Some(value), output)),

                            Ok(Step::Halt) if i == p_last => Err(Ok(output.unwrap())),
                            Ok(Step::Halt) => Ok((None, output)),
                            Err(e) => Err(Err(e)),
                            _ => unreachable!(),
                        }
                    })
                    .unwrap_err()
                    .and_then(|value| match value.cmp(&current_max) {
                        Ordering::Greater => Ok((value, p)),
                        _ => Ok((current_max, current_permutation)),
                    })
            },
        )
        .unwrap_or_else(|e| panic!("invalid state: {:?}", e))
}

pub fn part_1() -> Memory {
    solve_1(&DATA).0
}

pub fn part_2() -> Memory {
    solve_2(&DATA).0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_step() {
        let memory = parse(r#"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"#);

        let mut cpu = CPU::new(memory.to_owned(), 0, None);

        loop {
            match cpu.step() {
                Ok(Step::NeedInput) => break,
                Ok(Step::Continue) => {}
                state => panic!("invalid state {:?}", state),
            }
        }

        println!("test_step ip: {}", cpu.ip);

        assert_eq!(memory.last(), Some(&0));
    }

    #[test]
    fn test_step_with_input() {
        let memory = parse(r#"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"#);

        let mut cpu = CPU::new(memory, 0, Some(0));

        loop {
            match cpu.step() {
                Ok(Step::NeedInput) => break,
                Ok(Step::Continue) => {}
                Ok(Step::Output(value)) => {
                    println!("test_test_with_input output: {}", value);
                }
                state => panic!("invalid state {:?}", state),
            }
        }

        println!("test_test_with_input ip: {}", cpu.ip);

        assert_eq!(cpu.input, None);
    }

    #[test]
    fn test_example_1_1() {
        let base_memory = parse(r#"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"#);

        assert_eq!(solve_1(&base_memory), (43210, vec![4, 3, 2, 1, 0]));
    }

    #[test]
    fn test_example_1_2() {
        let base_memory =
            parse(r#"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"#);

        assert_eq!(solve_1(&base_memory), (54321, vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_example_1_3() {
        let base_memory = parse(
            r#"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"#,
        );

        assert_eq!(solve_1(&base_memory), (65210, vec![1, 0, 4, 3, 2]));
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_2_1() {
        let base_memory = parse(
            r#"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"#,
        );

        assert_eq!(solve_2(&base_memory), (139629729, vec![9, 8, 7, 6, 5]));
    }

    #[test]
    fn test_example_2_2() {
        let base_memory = parse(
            r#"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"#,
        );

        assert_eq!(solve_2(&base_memory), (18216, vec![9, 7, 8, 5, 6]));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
