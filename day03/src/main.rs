use std::time::Instant;

fn main() {
    let now = Instant::now();
    
    println!("part 1: {}", day03::part_1());
    println!("part 2: {}", day03::part_2());

    let elapsed = now.elapsed();
    
    println!("elapsed: {}ms ({}ns)", elapsed.as_millis(), elapsed.as_nanos());
}
