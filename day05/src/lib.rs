#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

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

fn parse(data: &str) -> Vec<Memory> {
    data.trim()
        .split(SEPARATOR)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<Memory>>()
}

lazy_static! {
    static ref DATA: Vec<Memory> = parse(include_str!("../data.txt"));
}

pub fn execute(
    mut input: Option<Memory>,
    istructions: &mut [Memory],
) -> Result<Vec<Memory>, Error> {
    let mut result = vec![];

    let mut ip = 0;
    loop {
        let opcode = istructions.get(ip).ok_or_else(|| Error::EOF)?.to_owned() as Opcode;

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
                    istructions,
                    ip + 3,
                    mode(3)?,
                    read(istructions, ip + 1, mode(1)?)? + read(istructions, ip + 2, mode(2)?)?,
                )?;
                ip += 4;
            }
            2 => {
                write(
                    istructions,
                    ip + 3,
                    mode(3)?,
                    read(istructions, ip + 1, mode(1)?)? * read(istructions, ip + 2, mode(2)?)?,
                )?;
                ip += 4;
            }
            3 => {
                write(
                    istructions,
                    ip + 1,
                    OpcodeMode::Positional,
                    input.take().ok_or_else(|| Error::NoInput(ip))?,
                )?;
                ip += 2;
            }
            4 => {
                result.push(read(istructions, ip + 1, mode(1)?)?);
                ip += 2;
            }
            5 => {
                ip = if read(istructions, ip + 1, mode(1)?)? != 0 {
                    read(istructions, ip + 2, mode(2)?)? as usize
                } else {
                    ip + 3
                }
            }
            6 => {
                ip = if read(istructions, ip + 1, mode(1)?)? == 0 {
                    read(istructions, ip + 2, mode(2)?)? as usize
                } else {
                    ip + 3
                }
            }
            7 => {
                let value = if read(istructions, ip + 1, mode(1)?)?
                    < read(istructions, ip + 2, mode(2)?)?
                {
                    1
                } else {
                    0
                };

                write(istructions, ip + 3, mode(3)?, value)?;
                ip += 4;
            }
            8 => {
                let value = if read(istructions, ip + 1, mode(1)?)?
                    == read(istructions, ip + 2, mode(2)?)?
                {
                    1
                } else {
                    0
                };

                write(istructions, ip + 3, mode(3)?, value)?;
                ip += 4;
            }
            99 => return Ok(result),
            _ => return Err(Error::InvalidOpcode(opcode, ip)),
        }
    }
}

pub fn part_1() -> Memory {
    execute(Some(1), &mut DATA.clone())
        .expect("err")
        .pop()
        .unwrap()
}

pub fn part_2() -> Memory {
    execute(Some(5), &mut DATA.clone())
        .expect("err")
        .pop()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_example_1() {
        let mut istructions = parse("3,0,4,0,99");

        let result = execute(Some(1), &mut istructions).expect("err");
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_part_1() {
        let mut values = execute(Some(1), &mut DATA.clone()).expect("err");
        assert!(values.pop().unwrap() != 0);
        assert!(values.iter().all(|&v| v == 0));
    }

    #[test]
    fn test_example_2_1() {
        let mut istructions = parse("3,9,8,9,10,9,4,9,99,-1,8");

        assert_eq!(execute(Some(8), &mut istructions).expect("err"), vec![1]);
    }

    #[test]
    fn test_example_2_2() {
        let mut istructions = parse("3,9,7,9,10,9,4,9,99,-1,8");

        assert_eq!(execute(Some(8), &mut istructions).expect("err"), vec![0]);
    }

    #[test]
    fn test_example_2_3() {
        let mut istructions = parse("3,3,1108,-1,8,3,4,3,99");

        assert_eq!(execute(Some(8), &mut istructions).expect("err"), vec![1]);
    }

    #[test]
    fn test_example_2_4() {
        let mut istructions = parse("3,3,1107,-1,8,3,4,3,99");

        assert_eq!(execute(Some(8), &mut istructions).expect("err"), vec![0]);
    }

    #[test]
    fn test_example_2_5() {
        let mut istructions = parse("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");

        assert_eq!(execute(Some(8), &mut istructions).expect("err"), vec![1]);
    }

    #[test]
    fn test_example_2_6() {
        let mut istructions = parse("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");

        assert_eq!(execute(Some(8), &mut istructions).expect("err"), vec![1]);
    }

    #[test]
    fn test_example_2_7() {
        let mut istructions = parse("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");

        assert_eq!(execute(Some(8), &mut istructions).expect("err"), vec![1000]);
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
