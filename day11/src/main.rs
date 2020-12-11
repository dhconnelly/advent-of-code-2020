#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    m: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn parse(s: &str) -> Self {
        let rows: usize = s.lines().count();
        let cols: usize = s.lines().next().unwrap().len();
        let m = s.lines().flat_map(|line| line.chars()).collect();
        Grid { m, rows, cols }
    }

    fn pt(&self, i: usize) -> Pt2 {
        Pt2::new((i / self.cols) as i32, (i % self.cols) as i32)
    }

    fn step<F>(&self, f: F) -> Self
    where F: Fn(&Pt2, char) -> char {
        let m = self
            .m
            .iter()
            .enumerate()
            .map(|(i, tile)| f(&self.pt(i), *tile))
            .collect();
        let (rows, cols) = (self.rows, self.cols);
        Self { m, rows, cols }
    }

    fn in_bounds(&self, pt: &Pt2) -> bool {
        pt.row >= 0
            && pt.row < self.rows as i32
            && pt.col >= 0
            && pt.col < self.cols as i32
    }

    fn get(&self, pt: &Pt2) -> char {
        let i = pt.row * self.cols as i32 + pt.col;
        self.m[i as usize]
    }

    fn step1(&self) -> Self {
        self.step(|pt, tile| {
            let occupied = SLOPES
                .iter()
                .map(|slope| pt.add(*slope))
                .filter(|q| self.in_bounds(q))
                .filter(|q| self.get(q) == '#')
                .count();
            let tile2 = match (tile, occupied) {
                ('L', 0) => '#',
                ('#', n) if n >= 4 => 'L',
                _ => tile,
            };
            tile2
        })
    }

    fn nbr_in_dir(&self, pt: &Pt2, slope: &(i32, i32)) -> Option<char> {
        let (drow, dcol) = slope;
        let apply = |pt: &Pt2| Pt2::new(pt.row + drow, pt.col + dcol);
        let mut pt2 = apply(&pt);
        while self.in_bounds(&pt2) && self.get(&pt2) == '.' {
            pt2 = apply(&pt2);
        }
        if !self.in_bounds(&pt2) {
            None
        } else {
            Some(self.get(&pt2))
        }
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
            tile2
        })
    }

    fn step_until_stable<F>(&self, f: F) -> Self
    where F: Fn(&Self) -> Self {
        let mut last = f(self);
        loop {
            let new = f(&last);
            if last.m.iter().enumerate().any(|(i, v)| new.m[i] != *v) {
                last = new;
                continue;
            }
            break;
        }
        last
    }
}

fn occupied(g: &Grid) -> usize {
    g.m.iter().filter(|v| **v == '#').count()
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
