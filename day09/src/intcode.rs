use std::collections::HashMap;
use std::ops::{Index, IndexMut, Deref};
use std::sync::mpsc::{Receiver, RecvError, SendError, Sender};
use std::thread;

#[derive(Debug)]
pub enum OpcodeMode {
    Positional,
    Immediate,
    Relative,
}

pub type Opcode = u128;
pub type Memory = i128;

#[derive(Debug)]
pub enum Error {
    InvalidOpcode(Opcode, usize),
    InvalidOpcodeMode(Opcode, OpcodeMode),
    InvalidOpcodeModeIndex(Opcode, u8),
    InvalidOpcodeModeValue(Opcode, u8),
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
    memory: MemoryData,
    ip: usize,
    base: i128,
    input: Option<Memory>,
}

#[derive(Clone)]
struct MemoryData(HashMap<usize, Memory>);

impl MemoryData {
    fn new(data: Vec<Memory>) -> Self {
        Self(data.into_iter().enumerate().collect())
    }
}

impl Deref for MemoryData {
    type Target = HashMap<usize, Memory>;

    fn deref(&self) -> &HashMap<usize, Memory> {
        &self.0
    }
}

impl Index<usize> for MemoryData {
    type Output = Memory;

    fn index(&self, idx: usize) -> &Memory {
        match self.0.get(&idx) {
            Some(value) => value,
            None => &0,
        }
    }
}

impl IndexMut<usize> for MemoryData {
    fn index_mut(&mut self, idx: usize) -> &mut Memory {
        self.0.entry(idx).or_insert(0)
    }
}

impl CPU {
    pub fn new(memory: Vec<Memory>, ip: usize, input: Option<Memory>) -> Self {
        Self {
            memory: MemoryData::new(memory),
            ip,
            base: 0,
            input,
        }
    }

    pub fn input(&self) -> Option<Memory> {
        self.input
    }

    pub fn set_input(&mut self, input: Option<Memory>) {
        self.input = input;
    }

    #[inline(always)]
    fn mode(&self, opcode: Opcode, i: u8) -> Result<OpcodeMode, Error> {
        match i {
            1 => self.opcode_mode(opcode, (opcode / 100 % 10) as u8),
            2 => self.opcode_mode(opcode, (opcode / 1000 % 10) as u8),
            3 => self.opcode_mode(opcode, (opcode / 10000 % 10) as u8),
            _ => Err(Error::InvalidOpcodeModeIndex(opcode, i)),
        }
    }

    #[inline(always)]
    fn opcode_mode(&self, opcode: Opcode, i: u8) -> Result<OpcodeMode, Error> {
        match i {
            0 => Ok(OpcodeMode::Positional),
            1 => Ok(OpcodeMode::Immediate),
            2 => Ok(OpcodeMode::Relative),
            _ => Err(Error::InvalidOpcodeModeValue(opcode, i)),
        }
    }

    #[inline(always)]
    fn write(
        &mut self,
        opcode: Opcode,
        index: usize,
        mode: OpcodeMode,
        value: Memory,
    ) -> Result<(), Error> {
        match mode {
            OpcodeMode::Positional => {
                let idx = self.memory[index] as usize;
                self.memory[idx] = value;
                Ok(())
            }
            OpcodeMode::Immediate => Err(Error::InvalidOpcodeMode(opcode, mode)),
            OpcodeMode::Relative => {
                let idx = (self.base + self.memory[index]) as usize;
                self.memory[idx] = value;
                Ok(())
            }
        }
    }

    #[inline(always)]
    fn read(&self, _opcode: Opcode, index: usize, mode: OpcodeMode) -> Result<Memory, Error> {
        let memory = &self.memory;
        match mode {
            OpcodeMode::Positional => Ok(memory[memory[index] as usize]),
            OpcodeMode::Immediate => Ok(memory[index]),
            OpcodeMode::Relative => Ok(memory[(memory[index] + self.base) as usize]),
        }
    }

    pub fn step(&mut self) -> Result<Step, Error> {
        let opcode = self
            .memory
            .get(&self.ip)
            .ok_or_else(|| Error::EOF)?
            .to_owned() as Opcode;

        match opcode % 100 {
            1 => {
                self.write(
                    opcode,
                    self.ip + 3,
                    self.mode(opcode, 3)?,
                    self.read(opcode, self.ip + 1, self.mode(opcode, 1)?)?
                        + self.read(opcode, self.ip + 2, self.mode(opcode, 2)?)?,
                )?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            2 => {
                self.write(
                    opcode,
                    self.ip + 3,
                    self.mode(opcode, 3)?,
                    self.read(opcode, self.ip + 1, self.mode(opcode, 1)?)?
                        * self.read(opcode, self.ip + 2, self.mode(opcode, 2)?)?,
                )?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            3 => {
                if let Some(input) = self.input {
                    self.write(opcode, self.ip + 1, self.mode(opcode, 1)?, input)?;
                    self.input = None;
                    self.ip += 2;

                    Ok(Step::Continue)
                } else {
                    Ok(Step::NeedInput)
                }
            }
            4 => {
                let output = self.read(opcode, self.ip + 1, self.mode(opcode, 1)?)?;
                self.ip += 2;

                Ok(Step::Output(output))
            }
            5 => {
                self.ip = if self.read(opcode, self.ip + 1, self.mode(opcode, 1)?)? != 0 {
                    self.read(opcode, self.ip + 2, self.mode(opcode, 2)?)? as usize
                } else {
                    self.ip + 3
                };

                Ok(Step::Continue)
            }
            6 => {
                self.ip = if self.read(opcode, self.ip + 1, self.mode(opcode, 1)?)? == 0 {
                    self.read(opcode, self.ip + 2, self.mode(opcode, 2)?)? as usize
                } else {
                    self.ip + 3
                };

                Ok(Step::Continue)
            }
            7 => {
                let value = if self.read(opcode, self.ip + 1, self.mode(opcode, 1)?)?
                    < self.read(opcode, self.ip + 2, self.mode(opcode, 2)?)?
                {
                    1
                } else {
                    0
                };

                self.write(opcode, self.ip + 3, self.mode(opcode, 3)?, value)?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            8 => {
                let value = if self.read(opcode, self.ip + 1, self.mode(opcode, 1)?)?
                    == self.read(opcode, self.ip + 2, self.mode(opcode, 2)?)?
                {
                    1
                } else {
                    0
                };

                self.write(opcode, self.ip + 3, self.mode(opcode, 3)?, value)?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            9 => {
                self.base += self.read(opcode, self.ip + 1, self.mode(opcode, 1)?)?;
                self.ip += 2;

                Ok(Step::Continue)
            }
            99 => Ok(Step::Halt),

            _ => Err(Error::InvalidOpcode(opcode, self.ip)),
        }
    }

    pub fn copy_with_input(&self, input: Option<Memory>) -> Self {
        Self {
            memory: self.memory.clone(),
            input,
            ..*self
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
            match cpu.run() {
                Ok(Step::NeedInput) => break,
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

    #[test]
    fn test_self_copy_program() {
        let memory = parse(r#"109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"#);

        let mut cpu = CPU::new(memory.to_owned(), 0, None);

        let mut output = vec![];
        loop {
            match cpu.run() {
                Ok(Step::NeedInput) => panic!("invalid input request"),
                Ok(Step::Output(value)) => output.push(value),
                Ok(Step::Halt) => break,
                state => panic!("invalid state {:?}", state),
            }
        }

        assert_eq!(output, memory);
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_should_output_number() {
        let memory = parse(r#"1102,34915192,34915192,7,4,7,99,0"#);

        let mut cpu = CPU::new(memory, 0, None);

        let mut output = vec![];
        loop {
            match cpu.run() {
                Ok(Step::NeedInput) => panic!("invalid input request"),
                Ok(Step::Output(value)) => output.push(value),
                Ok(Step::Halt) => break,
                state => panic!("invalid state {:?}", state),
            }
        }

        assert_eq!(output, vec![1219070632396864]);
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_should_output_middle_number() {
        let memory = parse(r#"104,1125899906842624,99"#);

        let mut cpu = CPU::new(memory, 0, None);

        let mut output = vec![];
        loop {
            match cpu.run() {
                Ok(Step::NeedInput) => panic!("invalid input request"),
                Ok(Step::Output(value)) => output.push(value),
                Ok(Step::Halt) => break,
                state => panic!("invalid state {:?}", state),
            }
        }

        assert_eq!(output, vec![1125899906842624]);
    }
}
