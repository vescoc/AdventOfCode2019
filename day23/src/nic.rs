use std::collections::VecDeque;

use intcode;

#[derive(Clone, Debug)]
pub struct Packet {
    pub destination: intcode::Memory,
    pub x: intcode::Memory,
    pub y: intcode::Memory,
}

impl Packet {
    pub fn new(buffer: &[intcode::Memory; 3]) -> Self {
        Self {
            destination: buffer[0],
            x: buffer[1],
            y: buffer[2],
        }
    }
}

pub enum State {
    Running,
    Halted,
    Send(Packet),
}

pub struct NIC {
    cpu: intcode::CPU,
    input: VecDeque<intcode::Memory>,
    buffer: [intcode::Memory; 3],
    buffer_index: usize,
    idle: bool,
}

impl NIC {
    pub fn new(id: usize, program: Vec<intcode::Memory>) -> Self {
        Self {
            cpu: intcode::CPU::new(program, 0, Some(id as intcode::Memory)),
            input: VecDeque::new(),
            buffer: [0, 0, 0],
            buffer_index: 0,
            idle: false,
        }
    }

    pub fn run(&mut self) -> State {
        match self.cpu.run().unwrap() {
            intcode::Run::Halt => State::Halted,
            intcode::Run::NeedInput => {
                let value = match self.input.pop_front() {
                    Some(value) => {
                        self.idle = false;
                        value
                    }
                    None => {
                        self.idle = true;
                        -1
                    }
                };

                self.cpu.set_input(Some(value));
                State::Running
            }
            intcode::Run::Output(value) => {
                self.buffer[self.buffer_index] = value;

                self.buffer_index += 1;
                if self.buffer_index == 3 {
                    self.buffer_index = 0;

                    State::Send(Packet::new(&self.buffer))
                } else {
                    State::Running
                }
            }
        }
    }

    pub fn send(&mut self, packet: &Packet) {
        self.input.push_back(packet.x);
        self.input.push_back(packet.y);
    }

    pub fn is_idle(&self) -> bool {
        self.idle && self.buffer_index == 0
    }
}
