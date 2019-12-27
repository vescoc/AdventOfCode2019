#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use intcode;
pub mod engine;

lazy_static! {
    static ref DATA: &'static str = include_str!("../data.txt");
    pub static ref PROGRAM: Vec<intcode::Memory> = intcode::parse(&DATA);
}

pub fn part_1() -> String {
    let mut cpu = intcode::CPU::new(PROGRAM.to_vec(), 0, None);

    // infinite loop
    // molten lava
    // photons
    // escape pod
    let mut output = Vec::new();
    let commands = vec![
    	"east",
    	"take mug",
	"inv",
	"drop mug",
	"take cicio",
	"drop cicio",
	"inv",
    	"north",
    	"take monolith",
    	"south",
    	"south",
    	"west",
    	"north",
    	"west",
    	"take bowl of rice",
    	"north",
    	"west",
    	"north",
    	"inv",
    ];
    // let commands = vec![
    // 	"west",
    // 	"take ornament",
    // 	"south",
    // 	"east",
    // 	"take weather machine",
    // 	"south",
    // 	"north",
    // 	"west",
    // 	"west",
    // 	"east",
    // 	"north",
    // 	"inv",
    // ];
    // let commands = vec![
    // 	"south",
    // 	// "take photons",
    // 	"inv",
    // ];

    {
	let commands = commands.join("\n");
	let mut i = commands.chars();
	
	loop {
	    match cpu.run().expect("invalid program") {
		intcode::Run::NeedInput => {
		    if let Some(c) = i.next() {
			cpu.set_input(Some(c as intcode::Memory));
			print!("{}", c);
		    } else {
			break;
		    }
		},
		intcode::Run::Output(value) => {
		    print!("{}", value as u8 as char);
		    output.push(value as u8 as char);
		}
		intcode::Run::Halt => {
		    println!("--- got halt");
		    break;
		}
	    }
	}
    }

    todo!()
}

pub fn part_2() -> u32 {
    todo!()
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
