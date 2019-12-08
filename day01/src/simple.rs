pub fn calculate_fuel(mass: u32) -> Option<u32> {
    mass.checked_div(3).and_then(|v| v.checked_sub(2))
}

pub fn calculate_total_fuel(mass: u32) -> Option<u32> {
    let mut total = 0;
    let mut current = mass;
    loop {
        match calculate_fuel(current) {
            Some(c) if c > 0 => {
                current = c;
                total += c;
            }
            _ => break Some(total),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;
    use test::Bencher;

    use crate::{part, DATA};

    #[test]
    fn example_1() {
        assert_eq!(calculate_fuel(12), Some(2));
        assert_eq!(calculate_fuel(14), Some(2));
        assert_eq!(calculate_fuel(1969), Some(654));
        assert_eq!(calculate_fuel(100756), Some(33583));
    }

    #[test]
    fn example_2() {
        assert_eq!(calculate_total_fuel(12), Some(2));
        assert_eq!(calculate_total_fuel(1969), Some(966));
        assert_eq!(calculate_total_fuel(100756), Some(50346));
    }

    #[bench]
    fn bench_example_1(b: &mut Bencher) {
        b.iter(|| calculate_fuel(100756))
    }

    #[bench]
    fn bench_example_2(b: &mut Bencher) {
        b.iter(|| calculate_total_fuel(100756))
    }

    #[bench]
    fn bench_part(b: &mut Bencher) {
        b.iter(|| part(&DATA, calculate_fuel));
        b.iter(|| part(&DATA, calculate_total_fuel));
    }
}
