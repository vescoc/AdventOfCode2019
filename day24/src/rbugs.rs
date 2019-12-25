use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
struct BugsLayer {
    value: u32,
    level: isize,
    inner: Option<usize>,
    outer: Option<usize>,
}

impl BugsLayer {
    fn new(level: isize, inner: Option<usize>, outer: Option<usize>) -> Self {
	Self {
	    value: 0,
	    level,
	    inner,
	    outer,
	}
    }

    fn count(&self) -> u32 {
	let mut sum = 0;
	for i in 0..25 {
	    if self.value & (1 << i) != 0 {
		sum += 1;
	    }
	}

	sum
    }

    fn left(&self) -> u32 {
        let mut value = 0;
        for i in 0..5 {
            value += if self.value & (1 << (i * 5)) != 0 { 1 } else { 0 };
        }

        value
    }

    fn right(&self) -> u32 {
        let mut value = 0;
        for i in 0..5 {
            value += if self.value & (1 << (i * 5 + 4)) != 0 { 1 } else { 0 };
        }
        
        value
    }

    fn top(&self) -> u32 {
        let mut value = 0;
        for i in 0..5 {
            value += if self.value & (1 << i) != 0 { 1 } else { 0 };
        }
        
        value
    }

    fn bottom(&self) -> u32 {
        let mut value = 0;
        for i in 0..5 {
            value += if self.value & (1 << (20 + i)) != 0 { 1 } else { 0 };
        }
            
        value
    }
}

#[derive(Clone)]
pub struct Bugs {
    layers: Vec<BugsLayer>,
    innermost: usize,
    outermost: usize,
}

impl Bugs {
    fn get(&self, layer: usize, x: i32, y: i32, dx: i32, dy: i32) -> u32 {
        let right = |layer: &BugsLayer| layer.right();
        let left = |layer: &BugsLayer| layer.left();
        let top = |layer: &BugsLayer| layer.top();
        let bottom = |layer: &BugsLayer| layer.bottom();

        let inner = |f: fn (&BugsLayer) -> u32| {
            match self.layers[layer].inner {
                Some(index) => f(&self.layers[index]),
                None => 0,
            }
        };

	let outer = |x: u32, y: u32| {
	    match self.layers[layer].outer {
		Some(index) => if self.layers[index].value & (1 << (y * 5 + x)) != 0 { 1 } else { 0 },
		None => 0,
	    }
	};

	let get = |x: u32, y: u32| {
	    if self.layers[layer].value & (1 << (y * 5 + x)) != 0 { 1 } else { 0 }
	};
        
        match (x + dx, y + dy) {
            (2, 2) => match (dx, dy) {
                (-1, 0) => inner(right),
                (1, 0) => inner(left),
                (0, 1) => inner(bottom),
                (0, -1) => inner(top),
                _ => unreachable!(),
            },
	    (x @ 0..=4, y @ 0..=4) => get(x as u32, y as u32),
	    (-1, _) => outer(1, 2),
	    (5, _) => outer(3, 2),
	    (_, -1) => outer(2, 1),
	    (_, 5) => outer(2, 3),
	    _ => unreachable!(),
        }
    }

    fn allocate_innermost(&mut self) {
	let innermost = self.innermost;
	if self.layers[innermost].value != 0 {
	    let index = self.layers.len();
	    let level = self.layers[innermost].level;
	    self.layers[innermost].inner = Some(index);

	    self.innermost = index;

	    self.layers.push(BugsLayer::new(level - 1, None, Some(innermost)));
	}
    }

    fn allocate_outermost(&mut self) {
	let outermost = self.outermost;
	if self.layers[outermost].value != 0 {
	    let index = self.layers.len();
	    let level = self.layers[outermost].level;
	    self.layers[outermost].outer = Some(index);

	    self.outermost = index;

	    self.layers.push(BugsLayer::new(level + 1, Some(outermost), None));
	}
    }

    pub fn count_bugs(&self) -> u32 {
	let mut sum = 0;
	
	let mut current = self.innermost;
	loop {
	    sum += self.layers[current].count();

	    if let Some(index) = self.layers[current].outer {
		current = index;
	    } else {
		break sum;
	    }
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
                        '.' | '?' => Ok('0'),
                        '#' => Ok('1'),
                        _ => Err(format!("invalid char {}", c)),
                    })
                })
                .rev()
                .collect::<Result<String, String>>()?,
            2,
        )
            .map(|value| {
                Self {
                    layers: vec![
                        BugsLayer {
                            value,
                            level: 0,
                            inner: None,
                            outer: None,
                        }
                    ],
		    innermost: 0,
		    outermost: 0,
                }
            })
            .map_err(|e| format!("{}", e))
    }
}

impl fmt::Display for Bugs {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
	let mut current = self.outermost;
	loop {
	    writeln!(fmt, "Depth {}:", -self.layers[current].level)?;
	    fmt.write_str(
		&(0..25)
		    .flat_map(|i| {
			(String::from(if i % 5 == 0 && i != 0 { "\n" } else { "" })
			 + if i == 25 / 2 { "?" } else if self.layers[current].value & (1 << i) != 0 { "#" } else { "." })
			    .chars()
			    .collect::<Vec<char>>()
		    })
		    .collect::<String>(),
	    )?;
	    writeln!(fmt, "")?;

	    if let Some(index) = self.layers[current].inner {
		current = index;
	    } else {
		break Ok(());
	    }
	}
    }
}

impl Iterator for Bugs {
    type Item = Bugs;

    fn next(&mut self) -> Option<Bugs> {
	self.allocate_innermost();
	self.allocate_outermost();
	
        let bugs = self.clone();

	let mut current = self.innermost;
	loop {
            for x in 0..5 {
		for y in 0..5 {
		    if x != 2 || y != 2 {
			match (
			    [(0, 1), (0, -1), (1, 0), (-1, 0)]
				.iter()
				.map(|(dx, dy)| bugs.get(current, x, y, *dx, *dy))
				.sum::<u32>(),
			    bugs.get(current, x, y, 0, 0),
			) {
			    (s, 1) if s != 1 => self.layers[current].value &= !(1 << (y * 5 + x)),
			    (s, 0) if s == 1 || s == 2 => self.layers[current].value |= 1 << (y * 5 + x),
			    _ => {}
			}
		    }
		}
            }

	    if let Some(index) = self.layers[current].outer {
		current = index;
	    } else {
		break;
	    }
	}

        Some(self.to_owned())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
	let data = r"....#
#..#.
#.?##
..#..
#....";
	
	let bugs = data.parse::<Bugs>().unwrap();

	assert_eq!(format!("{}", bugs), format!("Depth 0:\n{}\n", data));
    }
}
