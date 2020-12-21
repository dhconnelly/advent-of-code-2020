type Grid = Vec<Vec<char>>;

fn flip(grid: &Grid) -> Grid {
    grid.iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

fn rotate(grid: &Grid) -> Grid {
    let mut rotated = grid.clone();
    let n = grid.len();
    for row in 0..n {
        for col in 0..n {
            rotated[col][n - row - 1] = grid[row][col];
        }
    }
    rotated
}

#[derive(Clone)]
struct Tile {
    id: usize,
    m: Grid,
}

fn parse_grid(s: &str) -> Grid {
    s.lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn parse_id(s: &str) -> usize {
    let mut toks = s.split(' ');
    toks.next().unwrap();
    usize::from_str_radix(toks.next().unwrap(), 10).unwrap()
}

fn parse_tiles(s: &str) -> Vec<Tile> {
    s.split("\n\n")
        .map(|chunk| chunk.trim())
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| {
            let mut segs = chunk.split(":\n");
            let id = parse_id(segs.next().unwrap());
            let m = parse_grid(segs.next().unwrap());
            Tile { id, m }
        })
        .collect()
}

fn corner_product(result: &[TileSpec], width: usize) -> u64 {
    result[0].id as u64
        * result[width - 1].id as u64
        * result[result.len() - width].id as u64
        * result[result.len() - 1].id as u64
}

#[derive(Debug, Clone)]
struct TileSpec {
    id: usize,
    top: String,
    right: String,
    bottom: String,
    left: String,
    rot: usize,
    flipped: bool,
}

impl TileSpec {
    fn flip(&self) -> Self {
        Self {
            id: self.id,
            right: self.left.clone(),
            left: self.right.clone(),
            top: self.top.chars().rev().collect(),
            bottom: self.bottom.chars().rev().collect(),
            flipped: !self.flipped,
            rot: self.rot,
        }
    }

    fn rotate(&self) -> Self {
        Self {
            id: self.id,
            right: self.top.clone(),
            bottom: self.right.chars().rev().collect(),
            left: self.bottom.clone(),
            top: self.left.chars().rev().collect(),
            rot: (self.rot + 1) % 4,
            flipped: self.flipped,
        }
    }
}

fn make_specs(tiles: &[Tile]) -> Vec<TileSpec> {
    tiles
        .iter()
        .map(|tile| {
            let n = tile.m.len();
            TileSpec {
                id: tile.id,
                top: tile.m[0].iter().collect(),
                right: (0..n).map(|i| &tile.m[i][n - 1]).collect(),
                bottom: tile.m[n - 1].iter().collect(),
                left: (0..n).map(|i| &tile.m[i][0]).collect(),
                rot: 0,
                flipped: false,
            }
        })
        .collect()
}

fn fits(
    pos: usize,
    spec: &TileSpec,
    result: &[TileSpec],
    width: usize,
) -> bool {
    if pos >= width {
        // check above
        if spec.top != result[pos - width].bottom {
            return false;
        }
    }
    if pos % width > 0 {
        // check left
        if spec.left != result[pos - 1].right {
            return false;
        }
    }
    true
}

fn solve(
    pos: usize,
    width: usize,
    specs: &[TileSpec],
    used: &[bool],
    result: &Vec<TileSpec>,
) -> Option<Vec<TileSpec>> {
    if result.len() == specs.len() {
        return Some(result.clone());
    }
    for i in (0..specs.len()).filter(|i| !used[*i]) {
        let mut used: Vec<_> = used.iter().copied().collect();
        used[i] = true;
        let mut spec = specs[i].clone();
        for j in 0..8 {
            if j == 4 {
                spec = spec.flip();
            }
            spec = spec.rotate();
            let mut result = result.clone();
            if !fits(pos, &spec, &result, width) {
                continue;
            }
            result.push(spec.clone());
            if let Some(v) = solve(pos + 1, width, specs, &used, &result) {
                return Some(v.clone());
            }
        }
    }
    None
}

fn grids_for_specs(tiles: &[Tile], specs: &[TileSpec]) -> Vec<Grid> {
    let tiles_per_id: std::collections::HashMap<usize, &Tile> =
        tiles.iter().map(|tile| (tile.id, tile)).collect();
    specs
        .iter()
        .map(|spec| {
            let mut grid = tiles_per_id[&spec.id].m.clone();
            if spec.flipped {
                grid = flip(&grid);
            }
            for _ in 0..spec.rot {
                grid = rotate(&grid);
            }
            grid
        })
        .collect()
}

fn remove_borders(grid: &Grid) -> Grid {
    let mut grid1 = vec![];
    for row in 1..grid.len() - 1 {
        grid1.push(vec![]);
        for col in 1..grid[row].len() - 1 {
            grid1[row - 1].push(grid[row][col]);
        }
    }
    grid1
}

fn make_image(grids_per_side: usize, grids: &[Grid]) -> Grid {
    let mut grid = vec![];
    let tiles_per_grid = grids[0].len();
    for chunk in grids.chunks(grids_per_side) {
        for row in 0..tiles_per_grid {
            let mut line = Vec::new();
            for grid in chunk {
                line.extend(grid[row].clone());
            }
            grid.push(line);
        }
    }
    grid
}

fn print_grid(grid: &Grid) {
    for line in grid {
        let s: String = line.iter().collect();
        println!("{}", s);
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let tiles = parse_tiles(&text);

    let width = (tiles.len() as f64).sqrt() as usize;
    let specs = make_specs(&tiles);
    let n = specs.len();
    let result = solve(0, width, &specs, &vec![false; n], &vec![]).unwrap();
    println!("{}", corner_product(&result, width));

    let grids = grids_for_specs(&tiles, &result);
    let grids: Vec<_> = grids.iter().map(remove_borders).collect();
    let mut img = make_image(width, &grids);
    img = rotate(&img);
    img = flip(&img);
    print_grid(&img);
}
