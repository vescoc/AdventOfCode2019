use std::collections::HashMap;
use std::ops::{Deref, Index, IndexMut};
use std::sync::mpsc::{Receiver, RecvError, SendError, Sender};
use std::thread;
use std::str::FromStr;

pub type Memory = i128;

const SEPARATOR: char = ',';

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

enum ModeIndex {
    One,
    Two,
    Three,
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    IfNEq(Mode, Mode),
    IfEq(Mode, Mode),
    IfLess(Mode, Mode, Mode),
    IfEqTo(Mode, Mode, Mode),
    Base(Mode),
    Halt,
}

#[derive(Debug)]
pub enum DecodeOpcodeError {
    InvalidOpcode(Memory, usize),
    InvalidOpcodeModeValue(Memory, usize),
}

impl Opcode {
    fn from(memory: &Memory, ip: usize) -> Result<Opcode, DecodeOpcodeError> {
        let mode = |index| {
            let value = match index {
                ModeIndex::One => memory / 100 % 10,
                ModeIndex::Two => memory / 1000 % 10,
                ModeIndex::Three => memory / 10000 % 10,
            };

            match value {
                0 => Ok(Mode::Position),
                1 => Ok(Mode::Immediate),
                2 => Ok(Mode::Relative),
                _ => Err(DecodeOpcodeError::InvalidOpcodeModeValue(*memory, ip)),
            }
        };

        match memory % 100 {
            1 => Ok(Opcode::Add(
                mode(ModeIndex::One)?,
                mode(ModeIndex::Two)?,
                mode(ModeIndex::Three)?,
            )),
            2 => Ok(Opcode::Mul(
                mode(ModeIndex::One)?,
                mode(ModeIndex::Two)?,
                mode(ModeIndex::Three)?,
            )),
            3 => Ok(Opcode::Input(mode(ModeIndex::One)?)),
            4 => Ok(Opcode::Output(mode(ModeIndex::One)?)),
            5 => Ok(Opcode::IfNEq(mode(ModeIndex::One)?, mode(ModeIndex::Two)?)),
            6 => Ok(Opcode::IfEq(mode(ModeIndex::One)?, mode(ModeIndex::Two)?)),
            7 => Ok(Opcode::IfLess(
                mode(ModeIndex::One)?,
                mode(ModeIndex::Two)?,
                mode(ModeIndex::Three)?,
            )),
            8 => Ok(Opcode::IfEqTo(
                mode(ModeIndex::One)?,
                mode(ModeIndex::Two)?,
                mode(ModeIndex::Three)?,
            )),
            9 => Ok(Opcode::Base(mode(ModeIndex::One)?)),
            99 => Ok(Opcode::Halt),
            _ => Err(DecodeOpcodeError::InvalidOpcode(*memory, ip)),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidOpcode(DecodeOpcodeError),
    InvalidOpcodeMode(Opcode, Mode, usize),
    InvalidOpcodeModeValue(Memory, usize),
    EOF,
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

#[derive(Debug)]
pub enum Run {
    NeedInput,
    Output(Memory),
    Halt,
}

pub struct CPU {
    memory: MemoryData,
    ip: usize,
    base: Memory,
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
    fn write(
        &mut self,
        opcode: Opcode,
        index: usize,
        mode: Mode,
        value: Memory,
    ) -> Result<(), Error> {
        match mode {
            Mode::Position => {
                let idx = self.memory[index] as usize;
                self.memory[idx] = value;

                Ok(())
            }
            Mode::Immediate => Err(Error::InvalidOpcodeMode(opcode, mode, self.ip)),
            Mode::Relative => {
                let idx = (self.base + self.memory[index]) as usize;
                self.memory[idx] = value;

                Ok(())
            }
        }
    }

    #[inline(always)]
    fn read(&self, index: usize, mode: Mode) -> Result<Memory, Error> {
        let memory = &self.memory;
        match mode {
            Mode::Position => Ok(memory[memory[index] as usize]),
            Mode::Immediate => Ok(memory[index]),
            Mode::Relative => Ok(memory[(memory[index] + self.base) as usize]),
        }
    }

    pub fn step(&mut self) -> Result<Step, Error> {
        let opcode = Opcode::from(
            self.memory.get(&self.ip).ok_or_else(|| Error::EOF)?,
            self.ip,
        )
        .map_err(Error::InvalidOpcode)?;

        match opcode {
            Opcode::Add(mode1, mode2, mode3) => {
                self.write(
                    opcode,
                    self.ip + 3,
                    mode3,
                    self.read(self.ip + 1, mode1)? + self.read(self.ip + 2, mode2)?,
                )?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            Opcode::Mul(mode1, mode2, mode3) => {
                self.write(
                    opcode,
                    self.ip + 3,
                    mode3,
                    self.read(self.ip + 1, mode1)? * self.read(self.ip + 2, mode2)?,
                )?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            Opcode::Input(mode1) => {
                if let Some(input) = self.input {
                    self.write(opcode, self.ip + 1, mode1, input)?;
                    self.input = None;
                    self.ip += 2;

                    Ok(Step::Continue)
                } else {
                    Ok(Step::NeedInput)
                }
            }
            Opcode::Output(mode1) => {
                let output = self.read(self.ip + 1, mode1)?;
                self.ip += 2;

                Ok(Step::Output(output))
            }
            Opcode::IfNEq(mode1, mode2) => {
                self.ip = if self.read(self.ip + 1, mode1)? != 0 {
                    self.read(self.ip + 2, mode2)? as usize
                } else {
                    self.ip + 3
                };

                Ok(Step::Continue)
            }
            Opcode::IfEq(mode1, mode2) => {
                self.ip = if self.read(self.ip + 1, mode1)? == 0 {
                    self.read(self.ip + 2, mode2)? as usize
                } else {
                    self.ip + 3
                };

                Ok(Step::Continue)
            }
            Opcode::IfLess(mode1, mode2, mode3) => {
                let value = if self.read(self.ip + 1, mode1)? < self.read(self.ip + 2, mode2)? {
                    1
                } else {
                    0
                };

                self.write(opcode, self.ip + 3, mode3, value)?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            Opcode::IfEqTo(mode1, mode2, mode3) => {
                let value = if self.read(self.ip + 1, mode1)? == self.read(self.ip + 2, mode2)? {
                    1
                } else {
                    0
                };

                self.write(opcode, self.ip + 3, mode3, value)?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            Opcode::Base(mode1) => {
                self.base += self.read(self.ip + 1, mode1)?;
                self.ip += 2;

                Ok(Step::Continue)
            }
            Opcode::Halt => Ok(Step::Halt),
        }
    }

    pub fn copy_with_input(&self, input: Option<Memory>) -> Self {
        Self {
            memory: self.memory.clone(),
            input,
            ..*self
        }
    }

    pub fn run(&mut self) -> Result<Run, Error> {
        loop {
            match self.step()? {
                Step::Continue => {}
                Step::NeedInput => break Ok(Run::NeedInput),
                Step::Output(value) => break Ok(Run::Output(value)),
                Step::Halt => break Ok(Run::Halt),
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
                match self.run().map_err(ErrorSpawn::CPU)? {
                    Run::Halt => return Ok(()),
                    Run::NeedInput => {
                        self.input = Some(input_rx.recv().map_err(ErrorSpawn::Recv)?);
                    }
                    Run::Output(value) => {
                        output_tx.send(value).map_err(ErrorSpawn::Send)?;
                    }
                }
            }
        })
    }
}

pub fn parse<T: FromStr>(data: &str) -> Vec<T> {
    data.trim()
        .split(SEPARATOR)
        .map(|s| s.parse().unwrap_or_else(|_| panic!("cannot parse: {}", s)))
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
            match cpu.run().unwrap_or_else(|_| panic!("error")) {
                Run::NeedInput => break,
                Run::Output(value) => {
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
            match cpu.run().unwrap_or_else(|_| panic!("error")) {
                Run::NeedInput => panic!("invalid input request"),
                Run::Output(value) => output.push(value),
                Run::Halt => break,
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
            match cpu.run().unwrap_or_else(|_| panic!("error")) {
                Run::NeedInput => panic!("invalid input request"),
                Run::Output(value) => output.push(value),
                Run::Halt => break,
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
            match cpu.run().unwrap_or_else(|_| panic!("error")) {
                Run::NeedInput => panic!("invalid input request"),
                Run::Output(value) => output.push(value),
                Run::Halt => break,
            }
        }

        assert_eq!(output, vec![1125899906842624]);
    }
}
