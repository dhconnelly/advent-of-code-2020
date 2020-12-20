type Grid = Vec<Vec<char>>;

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

fn rotate(tile: &mut Tile) {
    let mut tile1 = tile.clone();
    for row in 0..tile.m.len() {
        let width = tile.m[row].len();
        for col in 0..width {
            tile1.m[row][col] = tile.m[col][width - row - 1];
        }
    }
    *tile = tile1;
}

fn flip(tile: &mut Tile) {
    for row in 0..tile.m.len() {
        let width = tile.m[row].len();
        for col in 0..width / 2 {
            let tmp = tile.m[row][col];
            tile.m[row][col] = tile.m[row][width - col - 1];
            tile.m[row][width - col - 1] = tmp;
        }
    }
}

fn fits_lr(left: &Tile, right: &Tile) -> bool {
    for row in 0..left.m.len() {
        if left.m[row][left.m[row].len() - 1] != right.m[row][0] {
            return false;
        }
    }
    true
}

fn fits_ab(above: &Tile, below: &Tile) -> bool {
    for col in 0..above.m[0].len() {
        if below.m[0][col] != above.m[above.m.len() - 1][col] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fits() {
        let left = parse_grid(
            &"#...##.#..
              ..#.#..#.# 
              .###....#.
              ###.##.##.
              .###.#####
              .##.#....#
              #...######
              .....#..##
              #.####...#
              #.##...##.",
        );

        let right = parse_grid(
            &"..###..### 
              ###...#.#. 
              ..#....#.. 
              .#.#.#..## 
              ##...#.### 
              ##.##.###. 
              ####.#...# 
              #...##..#. 
              ##..#..... 
              ..##.#..#.",
        );

        let below = parse_grid(
            &"#.##...##. 
              ##..#.##.. 
              ##.####... 
              ####.#.#.. 
              .#.####... 
              .##..##.#. 
              ....#..#.# 
              ..#.#..... 
              ####.#.... 
              ...#.#.#.#",
        );

        let mut left = Tile { m: left, id: 1 };
        let mut right = Tile { m: right, id: 2 };
        let mut below = Tile { m: below, id: 3 };

        assert!(fits_ab(&left, &below));
        rotate(&mut left);
        rotate(&mut left);
        rotate(&mut below);
        rotate(&mut below);
        assert!(fits_ab(&below, &left));

        rotate(&mut left);
        rotate(&mut left);
        assert!(fits_lr(&left, &right));
        rotate(&mut left);
        rotate(&mut right);
        assert!(fits_ab(&right, &left));
        rotate(&mut left);
        rotate(&mut right);
        assert!(fits_lr(&right, &left));
        rotate(&mut left);
        rotate(&mut right);
        assert!(fits_ab(&left, &right));
        rotate(&mut left);
        rotate(&mut right);
        assert!(fits_lr(&left, &right));
        flip(&mut left);
        flip(&mut right);
        assert!(fits_lr(&right, &left));
    }
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

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let tiles = parse_tiles(&text);
    let width = (tiles.len() as f64).sqrt() as usize;
    let specs = make_specs(&tiles);
    let n = specs.len();
    let result = solve(0, width, &specs, &vec![false; n], &vec![]).unwrap();
    println!("{}", corner_product(&result, width));
}
