#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use intcode;

lazy_static! {
    static ref PROGRAM: Vec<intcode::Memory> = intcode::parse(include_str!("../data.txt"));
}

pub fn part_1() -> intcode::Memory {
    let mut cpu = intcode::CPU::new(PROGRAM.to_vec(), 0, None);

    // must jump: !A & D
    // can jump: !B & D | !C & D = (!B | !C) & D = !(B & C) & D
    // jump: !A & D | (!B | !C) & D = (!A | !B | !C)) & D = !(A & B & C) & D
    let mut input = "OR A J\nAND B J\nAND C J\nNOT J J\nAND D J\nWALK\n".chars();
    let mut output = vec![];
    let mut result = None;
    loop {
        match cpu.run().expect("invalid program") {
            intcode::Run::NeedInput => {
                if let Some(c) = input.next() {
                    cpu.set_input(Some(c as intcode::Memory));
                } else {
                    panic!("EOF");
                }
            }
            intcode::Run::Output(value) if value < 255 => output.push(value as u8 as char),
            intcode::Run::Output(value) => result = Some(value),
            intcode::Run::Halt => break,
        }
    }

    match result {
        Some(value) => value,
        None => panic!("{}", output.into_iter().collect::<String>()),
    }
}

pub fn part_2() -> intcode::Memory {
    let mut cpu = intcode::CPU::new(PROGRAM.to_vec(), 0, None);

    // must jump: !A & D & (!E -> H)
    // can jump: (!B & D & (!E -> H)) | (!C & D & (!E -> H)) = (!B | !C) & D & (!E -> H)
    // jump: must jump | can jump = (!A | !B | !C) & (!E -> H) & D = !(A & B & C) & (!E -> H) & D = (!A | !B | !C) & (!E -> H) & D = !(A & B & C) & (E | H) & D
    let mut input =
        "OR A J\nAND B J\nAND C J\nNOT J J\nAND D J\nOR E T\nOR H T\nAND T J\nRUN\n".chars();
    let mut output = vec![];
    let mut result = None;
    loop {
        match cpu.run().expect("invalid program") {
            intcode::Run::NeedInput => {
                if let Some(c) = input.next() {
                    cpu.set_input(Some(c as intcode::Memory));
                } else {
                    panic!("EOF");
                }
            }
            intcode::Run::Output(value) if value < 255 => output.push(value as u8 as char),
            intcode::Run::Output(value) => result = Some(value),
            intcode::Run::Halt => break,
        }
    }

    match result {
        Some(value) => value,
        None => panic!("{}", output.into_iter().collect::<String>()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
