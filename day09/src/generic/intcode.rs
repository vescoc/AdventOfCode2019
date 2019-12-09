// NO COMPILE!

use std::collections::HashMap;
use std::ops::{Index, IndexMut, Deref, Div, Rem, Add, Mul, AddAssign};
use std::convert::{TryInto, TryFrom};

#[allow(unused_imports)]
use std::sync::mpsc::{Receiver, RecvError, SendError, Sender};
#[allow(unused_imports)]
use std::thread;
use std::fmt::Debug;
use std::cmp::{Ordering, Ord};

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    IfNEq(Mode, Mode),
    IfEq(Mode, Mode),
    IfLess(Mode, Mode, Mode),
    IfEqTo(Mode, Mode, Mode),
    Rel(Mode),
    Halt,
}

impl Opcode {
    fn new<Memory>(memory: Memory, index: usize) -> Result<Opcode, Error<Memory>>
        where Memory: Rem<Output=Memory> + TryFrom<i64> + Div<Output=Memory> + TryInto<i64> + Copy + TryInto<i64>
    {
        let m = |i| {
            match i {
                0 => Ok(Mode::Position),
                1 => Ok(Mode::Immediate),
                2 => Ok(Mode::Relative),
                _ => Err(Error::InvalidOpcode(memory, index)),
            }            
        };
        
        let mode = |i| {
            match i {
                1 => m(memory.div(100.try_into()).rem(10.into()).into()),
                2 => m(memory.div(1000.into()).rem(10.into()).into()),
                3 => m(memory.div(10000.into()).rem(10.into()).into()),
                _ => unreachable!(),
            }
        };
        
        match memory.rem(Memory::from(100)).into() {
            1 => Ok(Opcode::Add(mode(1)?, mode(2)?, mode(3)?)),
            2 => Ok(Opcode::Mul(mode(1)?, mode(2)?, mode(3)?)),
            3 => Ok(Opcode::Input(mode(1)?)),
            4 => Ok(Opcode::Output(mode(1)?)),
            5 => Ok(Opcode::IfNEq(mode(1)?, mode(2)?)),
            6 => Ok(Opcode::IfEq(mode(1)?, mode(2)?)),
            7 => Ok(Opcode::IfLess(mode(1)?, mode(2)?, mode(3)?)),
            8 => Ok(Opcode::IfEqTo(mode(1)?, mode(2)?, mode(3)?)),
            9 => Ok(Opcode::Rel(mode(1)?)),
            99 => Ok(Opcode::Halt),
            _ => Err(Error::InvalidOpcode(memory, index)),
        }
    }
}

#[derive(Debug)]
pub enum Error<Memory>
{
    InvalidOpcode(Memory, usize),
    InvalidOpcodeMode(Opcode, Mode),
    EOF,
}

#[derive(Debug)]
pub enum ErrorSpawn<Memory>
    where Memory: Debug
{
    CPU(Error<Memory>),
    Recv(RecvError),
    Send(SendError<Memory>),
}

#[derive(Debug)]
pub enum Step<Memory>
    where Memory: Debug
{
    Continue,
    NeedInput,
    Output(Memory),
    Halt,
}

#[derive(Clone)]
struct MemoryData<Memory>(HashMap<usize, Memory>);

impl<Memory> MemoryData<Memory> {
    fn new(data: Vec<Memory>) -> Self {
        Self(data.into_iter().enumerate().collect())
    }
}

impl<Memory> Deref for MemoryData<Memory> {
    type Target = HashMap<usize, Memory>;

    fn deref(&self) -> &HashMap<usize, Memory> {
        &self.0
    }
}

impl<Memory: Default + Copy + Sized> Index<usize> for MemoryData<Memory> {
    type Output = Memory;

    fn index(&self, idx: usize) -> &Memory {
        match self.0.get(&idx) {
            Some(value) => value,
            None => &Memory::default(),
        }
    }
}

impl<Memory: Default + Copy> IndexMut<usize> for MemoryData<Memory> {
    fn index_mut(&mut self, idx: usize) -> &mut Memory {
        self.0.entry(idx).or_default()
    }
}

pub struct CPU<Memory> {
    memory: MemoryData<Memory>,
    ip: usize,
    base: Memory,
    input: Option<Memory>,
}

impl<Memory> CPU<Memory>
    where Memory: Debug + Div<Memory, Output=Memory> + Add<Memory, Output=Memory> + Copy + TryInto<usize> + Default + From<i64> + Rem<Memory, Output=Memory> + Mul<Memory, Output=Memory> + PartialEq + Ord + Send + AddAssign + TryInto<i64>
{
    pub fn new(memory: Vec<Memory>, ip: usize, input: Option<Memory>) -> Self {
        Self {
            memory: MemoryData::new(memory),
            ip,
            base: Memory::default(),
            input,
        }
    }

    pub fn input(&self) -> Option<Memory> {
        self.input
    }

    pub fn set_input(&mut self, input: Option<Memory>) {
        self.input = input;
    }

    // #[inline(always)]
    // fn mode(&self, opcode: Opcode, i: u8) -> Result<OpcodeMode, Error<Opcode>> {
    //     match i {
    //         1 => self.opcode_mode(opcode, (opcode.div(100) % 10) as u8),
    //         2 => self.opcode_mode(opcode, (opcode / 1000 % 10) as u8),
    //         3 => self.opcode_mode(opcode, (opcode / 10000 % 10) as u8),
    //         _ => Err(Error::InvalidOpcodeModeIndex(opcode, i)),
    //     }
    // }

    // #[inline(always)]
    // fn opcode_mode(&self, opcode: Opcode, i: u8) -> Result<OpcodeMode, Error<Opcode>> {
    //     match i {
    //         0 => Ok(OpcodeMode::Positional),
    //         1 => Ok(OpcodeMode::Immediate),
    //         2 => Ok(OpcodeMode::Relative),
    //         _ => Err(Error::InvalidOpcodeModeValue(opcode, i)),
    //     }
    // }

    #[inline(always)]
    fn write(
        &mut self,
        opcode: Opcode,
        index: usize,
        mode: Mode,
        value: Memory,
    ) -> Result<(), Error<Memory>> {
        match mode {
            Mode::Position => {
                let idx = self.memory[index].try_into().unwrap_or_else(|_| panic!("cast"));
                self.memory[idx] = value;
                
                Ok(())
            }
            Mode::Immediate => Err(Error::InvalidOpcodeMode(opcode, mode)),
            Mode::Relative => {
                let idx: usize = self.base.add(self.memory[index]).try_into().unwrap_or_else(|_| panic!("cast"));
                self.memory[idx] = value;
                
                Ok(())
            }
        }
    }

    #[inline(always)]
    fn read(&self, _opcode: Opcode, index: usize, mode: Mode) -> Result<Memory, Error<Memory>> {
        let memory = &self.memory;
        match mode {
            Mode::Position => Ok(memory[memory[index].try_into().unwrap_or_else(|_| panic!("cast"))]),
            Mode::Immediate => Ok(memory[index]),
            Mode::Relative => Ok(memory[memory[index].add(self.base).try_into().unwrap_or_else(|_| panic!("cast"))]),
        }
    }

    pub fn step(&mut self) -> Result<Step<Memory>, Error<Memory>> {
        let opcode = self
            .memory
            .get(&self.ip)
            .map(|&m| Opcode::new(m, self.ip))
            .ok_or_else(|| Error::EOF)??;

        match opcode {
            Opcode::Add(mode1, mode2, mode3) => {
                self.write(
                    opcode,
                    self.ip + 3,
                    mode3,
                    self.read(opcode, self.ip + 1, mode1)?
                        + self.read(opcode, self.ip + 2, mode2)?,
                )?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            Opcode::Mul(mode1, mode2, mode3) => {
                self.write(
                    opcode,
                    self.ip + 3,
                    mode3,
                    self.read(opcode, self.ip + 1, mode1)?.mul(self.read(opcode, self.ip + 2, mode2)?),
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
                let output = self.read(opcode, self.ip + 1, mode1)?;
                self.ip += 2;

                Ok(Step::Output(output))
            }
            Opcode::IfNEq(mode1, mode2) => {
                self.ip = if self.read(opcode, self.ip + 1, mode1)? != Memory::default() {
                    self.read(opcode, self.ip + 2, mode2)?.into()
                } else {
                    self.ip + 3
                };

                Ok(Step::Continue)
            }
            Opcode::IfEq(mode1, mode2) => {
                self.ip = if self.read(opcode, self.ip + 1, mode1)? == Memory::default() {
                    self.read(opcode, self.ip + 2, mode2)?.into()
                } else {
                    self.ip + 3
                };

                Ok(Step::Continue)
            }
            Opcode::IfLess(mode1, mode2, mode3) => {
                let value = match self.read(opcode, self.ip + 1, mode1)?.cmp(&self.read(opcode, self.ip + 2, mode2)?) {
                    Ordering::Less => Memory::from(1),
                    _ => Memory::default(),
                };

                self.write(opcode, self.ip + 3, mode3, value)?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            Opcode::IfEqTo(mode1, mode2, mode3) => {
                let value = if self.read(opcode, self.ip + 1, mode1)?
                    == self.read(opcode, self.ip + 2, mode2)?
                {
                    Memory::from(1)
                } else {
                    Memory::default()
                };

                self.write(opcode, self.ip + 3, mode3, value)?;
                self.ip += 4;

                Ok(Step::Continue)
            }
            Opcode::Rel(mode1) => {
                self.base += self.read(opcode, self.ip + 1, mode1)?;
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

    pub fn run(&mut self) -> Result<Step<Memory>, Error<Memory>> {
        loop {
            match self.step() {
                Ok(Step::Continue) => {}
                r => break r,
            }
        }
    }

    // pub fn spawn(
    //     mut self,
    //     input_rx: Receiver<Memory>,
    //     output_tx: Sender<Memory>,
    // ) -> thread::JoinHandle<Result<(), ErrorSpawn<Memory>>> {
    //     thread::spawn(move || -> Result<(), ErrorSpawn<Memory>> {
    //         loop {
    //             match self.run() {
    //                 Ok(Step::Halt) => return Ok(()),
    //                 Ok(Step::NeedInput) => {
    //                     self.input = Some(input_rx.recv().map_err(ErrorSpawn::Recv)?);
    //                 }
    //                 Ok(Step::Output(value)) => {
    //                     output_tx.send(value).map_err(ErrorSpawn::Send)?;
    //                 }
    //                 Err(e) => {
    //                     return Err(ErrorSpawn::CPU(e));
    //                 }
    //                 _ => unreachable!(),
    //             }
    //         }
    //     })
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use crate::parse;

    #[test]
    fn test_step() {
        let memory = parse(r#"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"#);

        let mut cpu = CPU::<i128>::new(memory.to_owned(), 0, None);

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
