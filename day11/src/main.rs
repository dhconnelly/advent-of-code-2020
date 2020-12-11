use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt2 {
    row: i32,
    col: i32,
}

type Slope = (i32, i32);

impl Pt2 {
    fn new(row: i32, col: i32) -> Self {
        Pt2 { row, col }
    }

    fn add(&self, slope: Slope) -> Self {
        let (drow, dcol) = slope;
        Pt2::new(self.row + drow, self.col + dcol)
    }
}

const SLOPES: [Slope; 8] = [
    (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, 1), (1, 0), (1, -1),
];

struct Grid {
    m: HashMap<Pt2, char>,
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
                m.insert(Pt2::new(row as i32, col as i32), ch);
            }
        }
        Grid { m, rows, cols }
    }

    fn step<F>(&self, f: F) -> Self
    where F: Fn(&Pt2, char) -> (Pt2, char) {
        let m = self.m.iter().map(|(pt, tile)| f(pt, *tile)).collect();
        let (rows, cols) = (self.rows, self.cols);
        Self { m, rows, cols }
    }

    fn step1(&self) -> Self {
        self.step(|pt, tile| {
            let occupied = SLOPES
                .iter()
                .filter_map(|slope| self.m.get(&pt.add(*slope)))
                .filter(|ch| **ch == '#')
                .count();
            let tile2 = match (tile, occupied) {
                ('L', 0) => '#',
                ('#', n) if n >= 4 => 'L',
                _ => tile,
            };
            (*pt, tile2)
        })
    }

    fn nbr_in_dir(&self, pt: &Pt2, slope: &(i32, i32)) -> Option<char> {
        let (drow, dcol) = slope;
        let apply = |pt: &Pt2| Pt2::new(pt.row + drow, pt.col + dcol);
        let mut pt2 = apply(&pt);
        while let Some('.') = self.m.get(&pt2) {
            pt2 = apply(&pt2);
        }
        self.m.get(&pt2).copied()
    }

    fn step2(&self) -> Self {
        self.step(|pt, tile| {
            let occupied = SLOPES
                .iter()
                .filter_map(|slope| self.nbr_in_dir(pt, slope))
                .filter(|t| *t == '#')
                .count();
            let tile2 = match (tile, occupied) {
                ('L', 0) => '#',
                ('#', n) if n >= 5 => 'L',
                _ => tile,
            };
            (*pt, tile2)
        })
    }

    fn key(&self) -> String {
        let fmt = |row, col| self.m[&Pt2::new(row as i32, col as i32)];
        (0..self.rows)
            .flat_map(|row| (0..self.cols).map(move |col| fmt(row, col)))
            .collect()
    }

    fn step_until_stable<F>(&self, f: F) -> Self
    where F: Fn(&Self) -> Self {
        let mut seen = std::collections::HashSet::new();
        let mut grid = f(self);
        let mut k = grid.key();
        while !seen.contains(&k) {
            seen.insert(k);
            grid = f(&grid);
            k = grid.key();
        }
        grid
    }
}

fn occupied(g: &Grid) -> usize {
    g.m.values().filter(|v| **v == '#').count()
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let grid = Grid::parse(&text);

    let stable1 = grid.step_until_stable(|g| g.step1());
    println!("{}", occupied(&stable1));

    let stable2 = grid.step_until_stable(|g| g.step2());
    println!("{}", occupied(&stable2));
}
