use std::collections::HashMap;
use std::hash::Hash;

trait Pt: Hash + Eq + Copy {
    fn nbrs(self) -> Box<dyn Iterator<Item = Self>>;
    fn walk_space(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>>;
    fn expand_bounds(min: &Self, max: &Self) -> (Self, Self)
    where
        Self: Sized;
}

#[derive(Debug)]
struct Cube<T: Pt> {
    m: HashMap<T, char>,
    min: T,
    max: T,
}

impl<T: Pt> Cube<T> {
    fn active_nbrs(&self, pt: &T) -> usize {
        let mut n = 0;
        for pt1 in pt.nbrs() {
            if self.m.get(&pt1) == Some(&'#') {
                n += 1;
            }
        }
        n
    }

    fn active(&self) -> usize {
        self.m.values().filter(|ch| ch == &&'#').count()
    }

    fn step(&self) -> Self {
        let mut m = HashMap::new();
        let (min, max) = T::expand_bounds(&self.min, &self.max);
        for pt in T::walk_space(min, max) {
            let st = self.m.get(&pt).unwrap_or(&'.');
            let st1 = match (st, self.active_nbrs(&pt)) {
                ('#', 2) | ('#', 3) | ('.', 3) => '#',
                _ => '.',
            };
            m.insert(pt, st1);
        }
        Self { m, min, max }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();

    let cube3 = parse3(&text);
    let cube3 = cube3.step().step().step().step().step().step();
    println!("{}", cube3.active());

    let cube4 = parse4(&text);
    let cube4 = cube4.step().step().step().step().step().step();
    println!("{}", cube4.active());
}

// ==========================================================================
// Parsing

fn parse3(s: &str) -> Cube<Pt3> {
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
    let min = Pt3(0, 0, 0);
    let max = Pt3(xmax - 1, ymax - 1, 0);
    Cube { m, min, max }
}

fn parse4(s: &str) -> Cube<Pt4> {
    let z = 0;
    let w = 0;
    let mut m = HashMap::new();
    let mut ymax = 0;
    let mut xmax = 0;
    for (y, line) in s.lines().enumerate() {
        xmax = 0;
        for (x, ch) in line.chars().enumerate() {
            m.insert(Pt4(x as i64, y as i64, z, w), ch);
            xmax += 1;
        }
        ymax += 1;
    }
    let min = Pt4(0, 0, 0, 0);
    let max = Pt4(xmax - 1, ymax - 1, 0, 0);
    Cube { m, min, max }
}

// ==========================================================================
// Geometry

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt3(i64, i64, i64);

impl Pt for Pt3 {
    fn nbrs(self) -> Box<dyn Iterator<Item = Self>> {
        Box::new((-1..2).flat_map(move |dx| {
            (-1..2).flat_map(move |dy| {
                (-1..2).filter_map(move |dz| match (dx, dy, dz) {
                    (0, 0, 0) => None,
                    (dx, dy, dz) => {
                        Some(Self(self.0 + dx, self.1 + dy, self.2 + dz))
                    }
                })
            })
        }))
    }

    fn walk_space(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new((min.0..max.0 + 1).flat_map(move |x| {
            (min.1..max.1 + 1).flat_map(move |y| {
                (min.2..max.2 + 1).map(move |z| Self(x, y, z))
            })
        }))
    }

    fn expand_bounds(min: &Self, max: &Self) -> (Self, Self) {
        let min = Self(min.0 - 1, min.1 - 1, min.2 - 1);
        let max = Self(max.0 + 1, max.1 + 1, max.2 + 1);
        (min, max)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt4(i64, i64, i64, i64);

impl Pt for Pt4 {
    fn nbrs(self) -> Box<dyn Iterator<Item = Self>> {
        Box::new((-1..2).flat_map(move |dx| {
            (-1..2).flat_map(move |dy| {
                (-1..2).flat_map(move |dz| {
                    (-1..2).filter_map(move |dw| match (dx, dy, dz, dw) {
                        (0, 0, 0, 0) => None,
                        (dx, dy, dz, dw) => Some(Self(
                            self.0 + dx,
                            self.1 + dy,
                            self.2 + dz,
                            self.3 + dw,
                        )),
                    })
                })
            })
        }))
    }

    fn walk_space(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new((min.0..max.0 + 1).flat_map(move |x| {
            (min.1..max.1 + 1).flat_map(move |y| {
                (min.2..max.2 + 1).flat_map(move |z| {
                    (min.3..max.3 + 1).map(move |w| Self(x, y, z, w))
                })
            })
        }))
    }

    fn expand_bounds(min: &Self, max: &Self) -> (Self, Self) {
        let min = Self(min.0 - 1, min.1 - 1, min.2 - 1, min.3 - 1);
        let max = Self(max.0 + 1, max.1 + 1, max.2 + 1, max.3 + 1);
        (min, max)
    }
}
