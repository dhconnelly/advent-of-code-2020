use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt2 {
    row: i32,
    col: i32,
}

impl Pt2 {
    fn new(row: usize, col: usize) -> Self {
        Pt2 { row: row as i32, col: col as i32 }
    }

    fn nbrs(&self) -> Vec<Pt2> {
        let mut nbrs = Vec::new();
        nbrs.push(Pt2 {
            row: self.row,
            col: self.col + 1,
        });
        nbrs.push(Pt2 {
            row: self.row + 1,
            col: self.col,
        });
        nbrs.push(Pt2 {
            row: self.row + 1,
            col: self.col + 1,
        });
        nbrs.push(Pt2 {
            row: self.row - 1,
            col: self.col,
        });
        nbrs.push(Pt2 {
            row: self.row - 1,
            col: self.col + 1,
        });
        nbrs.push(Pt2 {
            row: self.row,
            col: self.col - 1,
        });
        nbrs.push(Pt2 {
            row: self.row + 1,
            col: self.col - 1,
        });
        nbrs.push(Pt2 {
            row: self.row - 1,
            col: self.col - 1,
        });
        nbrs
    }
}

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
                m.insert(Pt2::new(row, col), ch);
            }
        }
        Grid { m, rows, cols }
    }

    fn iter(&self) -> Self {
        let m = self
            .m
            .iter()
            .map(|(pt, tile)| {
                let occ = pt
                    .nbrs()
                    .iter()
                    .filter(|pt2| self.m.get(pt2) == Some(&'#'))
                    .count();
                let tile2 = match (tile, occ) {
                    ('L', 0) => '#',
                    ('#', n) if n >= 4 => 'L',
                    _ => *tile,
                };
                (*pt, tile2)
            })
            .collect();
        Self {
            m,
            rows: self.rows,
            cols: self.cols,
        }
    }

    fn nbr_in_dir(&self, pt: &Pt2, slope: &(i32, i32)) -> Option<char> {
        let (drow, dcol) = slope;
        let apply = |pt: &Pt2| Pt2 {
            row: pt.row as i32 + drow,
            col: pt.col as i32 + dcol,
        };
        let mut pt2 = apply(&pt);
        while let Some('.') = self.m.get(&pt2) {
            pt2 = apply(&pt2);
        }
        self.m.get(&pt2).copied()
    }

    fn dir_nbrs(&self, pt: &Pt2) -> Vec<char> {
        let nbrs = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
        ]
        .iter()
        .filter_map(|slope| self.nbr_in_dir(pt, slope))
        .collect();
        nbrs
    }

    fn iter2(&self) -> Self {
        let m = self
            .m
            .iter()
            .map(|(pt, tile)| {
                let occ = self
                    .dir_nbrs(pt)
                    .iter()
                    .filter(|t| t == &&'#')
                    .count();
                let tile2 = match (tile, occ) {
                    ('L', 0) => '#',
                    ('#', n) if n >= 5 => 'L',
                    _ => *tile,
                };
                (*pt, tile2)
            })
            .collect();
        Self {
            m,
            rows: self.rows,
            cols: self.cols,
        }
    }

    fn iter_until_stable<F>(&self, f: F) -> Self
    where
        F: Fn(&Self) -> Self,
    {
        let mut seen = std::collections::HashSet::new();
        let mut grid = f(self);
        loop {
            let k = format!("{}", grid);
            if seen.contains(&k) {
                break;
            }
            seen.insert(k);
            grid = f(&grid);
        }
        grid
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(
                    f,
                    "{}",
                    self.m[&Pt2 {
                        row: row as i32,
                        col: col as i32,
                    }]
                )?;
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

    let stable = grid.iter_until_stable(|g| g.iter());
    println!(
        "{}",
        stable.m.values().filter(|v| **v == '#').count()
    );

    let stable = grid.iter_until_stable(|g| g.iter2());
    println!(
        "{}",
        stable.m.values().filter(|v| **v == '#').count()
    );
}
