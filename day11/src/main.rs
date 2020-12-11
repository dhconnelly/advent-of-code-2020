use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl Tile {
    fn parse(ch: char) -> Self {
        match ch {
            '.' => Tile::Floor,
            '#' => Tile::Occupied,
            'L' => Tile::Empty,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            Tile::Floor => '.',
            Tile::Occupied => '#',
            Tile::Empty => 'L',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt2 {
    row: usize,
    col: usize,
}

struct Grid {
    m: HashMap<Pt2, Tile>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn parse(s: &str) -> Self {
        let rows: usize = s.lines().count();
        let cols: usize = s.lines().next().unwrap().len();
        let mut m = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                m.insert(Pt2 { row, col }, Tile::parse(ch));
            }
        }
        Grid { m, rows, cols }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.m[&Pt2 { row, col }])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let grid = Grid::parse(&text);
    println!("{}", grid);
}
