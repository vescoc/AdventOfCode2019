use std::sync::mpsc::{Receiver, RecvError, SendError, Sender};
use std::thread;

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
pub enum ErrorSpawn {
    CPU(Error),
    Recv(RecvError),
    Send(SendError<Memory>),
}

#[derive(Debug)]
pub enum Step {
    Continue,
    NeedInput,
    Output(Memory),
    Halt,
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

    pub fn input(&self) -> Option<Memory> {
        self.input
    }

    pub fn set_input(&mut self, input: Option<Memory>) {
        self.input = input;
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

    pub fn copy_with_input(&self, input: Option<Memory>) -> Self {
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

    pub fn spawn(
        mut self,
        input_rx: Receiver<Memory>,
        output_tx: Sender<Memory>,
    ) -> thread::JoinHandle<Result<(), ErrorSpawn>> {
        thread::spawn(move || -> Result<(), ErrorSpawn> {
            loop {
                match self.run() {
                    Ok(Step::Halt) => return Ok(()),
                    Ok(Step::NeedInput) => {
                        self.input = Some(input_rx.recv().map_err(ErrorSpawn::Recv)?);
                    }
                    Ok(Step::Output(value)) => {
                        output_tx.send(value).map_err(ErrorSpawn::Send)?;
                    }
                    Err(e) => {
                        return Err(ErrorSpawn::CPU(e));
                    }
                    _ => unreachable!(),
                }
            }
        })
    }
}

const SEPARATOR: char = ',';

pub fn parse(data: &str) -> Vec<Memory> {
    data.trim()
        .split(SEPARATOR)
        .map(|s| {
            s.parse()
                .unwrap_or_else(|e| panic!("cannot parse: {}, {}", s, e))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

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
    fn test_spawn() {
        let memory = parse(r#"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"#);

        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();

        let cpu = CPU::new(memory, 0, Some(0)).spawn(input_rx, output_tx);

        input_tx.send(0).expect("send problem");

        assert_eq!(output_rx.recv().expect("recv problem"), 0);

        cpu.join()
            .expect("paniched")
            .unwrap_or_else(|e| panic!("error: {:?}", e));
    }
}
