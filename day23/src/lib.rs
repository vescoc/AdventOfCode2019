#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use intcode;

mod nic;

lazy_static! {
    static ref PROGRAM: Vec<intcode::Memory> = intcode::parse(include_str!("../data.txt"));
}

pub fn part_1() -> intcode::Memory {
    let mut cpus = (0..50)
        .map(|i| nic::NIC::new(i, PROGRAM.to_vec()))
        .collect::<Vec<nic::NIC>>();

    loop {
        let mut packets = vec![];

        for (i, cpu) in cpus.iter_mut().enumerate() {
            match cpu.run() {
                nic::State::Send(nic::Packet {
                    destination: 255,
                    y,
                    ..
                }) => return y,
                nic::State::Send(packet) => packets.push(packet),
                nic::State::Running => {}
                nic::State::Halted => panic!("cpu {} halted", i),
            }
        }

        packets
            .into_iter()
            .for_each(|packet| cpus[packet.destination as usize].send(&packet));
    }
}

pub fn part_2() -> intcode::Memory {
    let mut saved_nat_packet = None;
    let mut nat_packet = None;

    let mut cpus = (0..50)
        .map(|i| nic::NIC::new(i, PROGRAM.to_vec()))
        .collect::<Vec<nic::NIC>>();

    loop {
        let mut packets = vec![];

        for (i, cpu) in cpus.iter_mut().enumerate() {
            match cpu.run() {
                nic::State::Send(nic::Packet {
                    destination: 255,
                    x,
                    y,
                }) => {
                    nat_packet = Some(nic::Packet {
                        destination: 0,
                        x,
                        y,
                    })
                }
                nic::State::Send(packet) => packets.push(packet),
                nic::State::Running => {}
                nic::State::Halted => panic!("cpu {} halted", i),
            }
        }

        if packets.is_empty() && cpus.iter().all(|cpu| cpu.is_idle()) {
            if let Some(packet) = nat_packet.take() {
                saved_nat_packet = Some(packet.to_owned());
                packets.push(packet);
            } else if let Some(packet) = saved_nat_packet {
                return packet.y;
            }
        }

        packets
            .into_iter()
            .for_each(|packet| cpus[packet.destination as usize].send(&packet));
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
