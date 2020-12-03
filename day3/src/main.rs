use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Tree,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt2 {
    x: i64,
    y: i64,
}

impl Pt2 {
    fn add(&self, pt: &Pt2) -> Pt2 {
        Pt2 {
            x: self.x + pt.x,
            y: self.y + pt.y,
        }
    }
}

#[derive(Debug)]
struct Grid {
    tiles: HashMap<Pt2, Tile>,
    width: i64,
    height: i64,
}

impl Grid {
    fn at(&self, pt: &Pt2) -> Tile {
        let (x, y) = (pt.x % self.width, pt.y);
        self.tiles[&Pt2 { x, y }]
    }
}

fn parse(s: &str) -> Result<Grid, String> {
    let mut tiles = HashMap::new();
    let (mut width, mut height) = (0, 0);
    for (y, line) in s.lines().enumerate() {
        width = 0;
        for (x, ch) in line.chars().enumerate() {
            let pt = Pt2 { x: x as i64, y: y as i64 };
            let tile = match ch {
                '.' => Tile::Empty,
                '#' => Tile::Tree,
                ch => return Err(format!("bad tile at {}, {}: {}", x, y, ch)),
            };
            tiles.insert(pt, tile);
            width += 1;
        }
        height += 1;
    }
    Ok(Grid { tiles, width, height })
}

fn trees(g: &Grid, slope: &Pt2) -> usize {
    let mut p = Pt2 { x: 0, y: 0 };
    let mut n = 0;
    while p.y < g.height {
        if g.at(&p) == Tile::Tree {
            n += 1;
        }
        p = p.add(slope);
    }
    n
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let grid = parse(&text).unwrap();
    println!("{}", trees(&grid, &Pt2 { x: 3, y: 1 }));
    
    let slopes = vec![
        Pt2 { x: 1, y: 1 },
        Pt2 { x: 3, y: 1 },
        Pt2 { x: 5, y: 1 },
        Pt2 { x: 7, y: 1 },
        Pt2 { x: 1, y: 2 },
    ];
    let prod: usize = slopes.iter().map(|slope| trees(&grid, slope)).product();
    println!("{}", prod);
}
