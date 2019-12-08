use std::iter;

pub fn calculate_fuel(mass: u32) -> impl Iterator<Item = u32> {
    iter::once(mass / 3 - 2)
}

pub fn calculate_total_fuel(mass: u32) -> impl Iterator<Item = u32> {
    let f = |mass: &u32| mass.checked_div(3).and_then(|v| v.checked_sub(2));

    iter::successors(f(&mass), f)
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;
    use test::Bencher;

    use crate::{part, DATA};

    #[test]
    fn example_1() {
        assert_eq!(calculate_fuel(12).next(), Some(2));
        assert_eq!(calculate_fuel(14).next(), Some(2));
        assert_eq!(calculate_fuel(1969).next(), Some(654));
        assert_eq!(calculate_fuel(100756).next(), Some(33583));
    }

    #[test]
    fn example_2() {
        assert_eq!(calculate_total_fuel(12).sum::<u32>(), 2);
        assert_eq!(calculate_total_fuel(1969).sum::<u32>(), 966);
        assert_eq!(calculate_total_fuel(100756).sum::<u32>(), 50346);
    }

    #[bench]
    fn bench_parts(b: &mut Bencher) {
        b.iter(|| part(&DATA, calculate_fuel));
        b.iter(|| part(&DATA, calculate_total_fuel));
    }
}
