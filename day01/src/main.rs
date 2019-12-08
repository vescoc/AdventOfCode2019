use std::time::Instant;

use day01::{part, simple::calculate_fuel, simple::calculate_total_fuel, DATA};

fn main() {
    let now = Instant::now();

    println!("part 1: {}", part(&DATA, calculate_fuel));
    println!("part 2: {}", part(&DATA, calculate_total_fuel));

    let elapsed = now.elapsed();

    println!(
        "elapsed: {}ms ({}ns)",
        elapsed.as_millis(),
        elapsed.as_nanos()
    );
}
