use std::str::FromStr;
use std::cell::Cell;

#[derive(Clone)]
struct BugsLayer {
    value: u32,
    index: usize,
    inner: Option<usize>,
    outher: Option<usize>,
    left: Cell<Option<u32>>,
    right: Cell<Option<u32>>,
    top: Cell<Option<u32>>,
    bottom: Cell<Option<u32>>,
}

impl BugsLayer {
    fn left(&self) -> u32 {
        if let Some(value) = self.left.get() {
            value
        } else if self.value == 0 {
            self.left.set(Some(0));
            0
        } else {
            let mut value = 0;
            for i in 0..5 {
                value += if self.value & (1 << (i * 5)) != 0 { 1 } else { 0 };
            }

            self.left.set(Some(value));

            value
        }
    }

    fn right(&self) -> u32 {
        if let Some(value) = self.right.get() {
            value
        } else if self.value == 0 {
            self.right.set(Some(0));

            0
        } else {
            let mut value = 0;
            for i in 0..5 {
                value += if self.value & (1 << (i * 5 + 4)) != 0 { 1 } else { 0 };
            }
            
            self.right.set(Some(value));

            value
        }
    }

    fn top(&self) -> u32 {
        if let Some(value) = self.top.get() {
            value
        } else if self.value == 0 {
            self.top.set(Some(0));

            0
        } else {
            let mut value = 0;
            for i in 0..5 {
                value += if self.value & (1 << i) != 0 { 1 } else { 0 };
            }
            
            self.top.set(Some(value));

            value
        }
    }

    fn bottom(&self) -> u32 {
        if let Some(value) = self.bottom.get() {
            value
        } else if self.value == 0 {
            self.bottom.set(Some(0));

            0
        } else {
            let mut value = 0;
            for i in 0..5 {
                value += if self.value & (1 << (20 + i)) != 0 { 1 } else { 0 };
            }
            
            self.bottom.set(Some(value));

            value
        }
    }
}

struct Bugs(Vec<BugsLayer>);

impl Bugs {
    fn get(&self, layer: usize, x: i32, y: i32, dx: i32, dy: i32) -> u32 {
        let right = |layer: &BugsLayer| layer.right();
        let left = |layer: &BugsLayer| layer.left();
        let top = |layer: &BugsLayer| layer.top();
        let bottom = |layer: &BugsLayer| layer.bottom();

        let inner = |f: fn (&BugsLayer) -> u32| {
            match self.0[layer].inner {
                Some(index) => f(&self.0[index]),
                None => 0,
            }
        };
        
        match (x, y) {
            (2, 2) => match (dx, dy) {
                (-1, 0) => inner(right),
                (1, 0) => inner(left),
                (0, 1) => inner(bottom),
                (0, -1) => inner(top),
                _ => unreachable!(),
            },
        }
    }
}

impl FromStr for Bugs {
    type Err = String;

    fn from_str(data: &str) -> Result<Self, String> {
        u32::from_str_radix(
            &data
                .lines()
                .flat_map(|line| {
                    line.trim().chars().map(|c| match c {
                        '.' => Ok('0'),
                        '#' => Ok('1'),
                        _ => Err(format!("invalid char {}", c)),
                    })
                })
                .rev()
                .collect::<Result<String, String>>()?,
            2,
        )
            .map(|value| {
                Self(
                    vec![
                        BugsLayer {
                            value,
                            index: 0,
                            inner: None,
                            outher: None,
                            left: Cell::new(None),
                            right: Cell::new(None),
                            top: Cell::new(None),
                            bottom: Cell::new(None),
                        }
                    ]
                )
            })
        .map_err(|e| format!("{}", e))
    }
}
