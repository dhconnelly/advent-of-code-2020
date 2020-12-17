use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pt3(i64, i64, i64);

#[derive(Debug)]
struct Cube {
    m: HashMap<Pt3, char>,
    xdim: (i64, i64),
    ydim: (i64, i64),
    zdim: (i64, i64),
}

impl Cube {
    fn parse(s: &str) -> Self {
        let z = 0;
        let mut m = HashMap::new();
        let mut ymax = 0;
        let mut xmax = 0;
        for (y, line) in s.lines().enumerate() {
            xmax = 0;
            for (x, ch) in line.chars().enumerate() {
                m.insert(Pt3(x as i64, y as i64, z), ch);
                xmax += 1;
            }
            ymax += 1;
        }
        let xdim = (0, xmax - 1);
        let ydim = (0, ymax - 1);
        let zdim = (0, 0);
        Self {
            m,
            xdim,
            ydim,
            zdim,
        }
    }

    fn active_nbrs(&self, pt: &Pt3) -> usize {
        let mut n = 0;
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in &[-1, 0, 1] {
                    let pt1 = Pt3(pt.0 + dx, pt.1 + dy, pt.2 + dz);
                    if &pt1 == pt {
                        continue;
                    }
                    if self.m.get(&pt1) == Some(&'#') {
                        n += 1;
                    }
                }
            }
        }
        n
    }

    fn active(&self) -> usize {
        self.m.values().filter(|ch| ch == &&'#').count()
    }

    fn step(&self) -> Self {
        let mut m = HashMap::new();
        for x in self.xdim.0 - 1..self.xdim.1 + 2 {
            for y in self.ydim.0 - 1..self.ydim.1 + 2 {
                for z in self.zdim.0 - 1..self.zdim.1 + 2 {
                    let pt = Pt3(x, y, z);
                    let st = self.m.get(&pt).unwrap_or(&'.');
                    let st1 = match (st, self.active_nbrs(&pt)) {
                        ('#', 2) | ('#', 3) | ('.', 3) => '#',
                        _ => '.',
                    };
                    m.insert(pt, st1);
                }
            }
        }
        let xdim = (self.xdim.0 - 1, self.xdim.1 + 1);
        let ydim = (self.ydim.0 - 1, self.ydim.1 + 1);
        let zdim = (self.zdim.0 - 1, self.zdim.1 + 1);
        Self {
            m,
            xdim,
            ydim,
            zdim,
        }
    }
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for z in self.zdim.0..=self.zdim.1 {
            println!("z={}", z);
            for y in self.ydim.0..=self.ydim.1 {
                for x in self.xdim.0..=self.xdim.1 {
                    let pt = Pt3(x, y, z);
                    let ch = self.m.get(&pt).unwrap_or(&'.');
                    write!(f, "{}", ch)?;
                }
                write!(f, "\n")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let cube = Cube::parse(&text);
    let cube = cube.step().step().step().step().step().step();
    println!("{}", cube.active())
}
