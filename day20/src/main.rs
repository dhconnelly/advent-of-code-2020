use std::collections::HashMap;

type Grid = Vec<Vec<char>>;

#[derive(Clone)]
struct Tile {
    id: usize,
    m: Grid,
    flipped: bool,
    rot: usize,
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
            Tile {
                id,
                m,
                flipped: false,
                rot: 0,
            }
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
    tile.rot = tile.rot % 4;
}

fn flip(tile: &mut Tile) {
    tile.flipped = !tile.flipped;
    for row in 0..tile.m.len() {
        let width = tile.m[row].len();
        for col in 0..width / 2 {
            let tmp = tile.m[row][col];
            tile.m[row][col] = tile.m[row][width - col - 1];
            tile.m[row][width - col - 1] = tmp;
        }
    }
}

fn key(prefix: &str, tile1: &Tile, tile2: &Tile) -> String {
    format!(
        "{}-{}-{}-{}-{}-{}-{}",
        prefix,
        tile1.id,
        tile1.flipped,
        tile1.rot,
        tile2.id,
        tile2.flipped,
        tile2.rot
    )
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

fn fits(
    width: usize,
    tiles: &[Tile],
    tile_index: usize,
    at_index: usize,
    in_result: &[usize],
    fit_memo: &mut HashMap<String, bool>,
) -> bool {
    let (row, col) = (at_index / width, at_index % width);
    if row > 1 {
        // check the top row of tiles[(row, col)] against the bottom row
        // of in_result[(row-1, col)]
        let below = &tiles[tile_index];
        let above = &tiles[at_index - width];
        let k = key("ab", below, above);
        let v = fit_memo.get(&k);
        if Some(&false) == v {
            return false;
        } else if Some(&true) == v {
            // skip check
        } else {
            let v = fits_ab(above, below);
            fit_memo.insert(k, v);
            if !v {
                return false;
            }
        }
    }

    if col > 1 {
        // check the left col of tiles[(row, col)] against the right col
        // of in_result[(row, col-1)]
        let right = &tiles[tile_index];
        let left = &tiles[in_result[at_index - 1]];
        let k = key("lr", right, left);
        let v = fit_memo.get(&k);
        if Some(&false) == fit_memo.get(&k) {
            return false;
        } else if Some(&true) == v {
            // skip check
        } else {
            let v = fits_lr(left, right);
            fit_memo.insert(k, v);
            if !v {
                return false;
            }
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

        let mut left = Tile {
            m: left,
            id: 1,
            rot: 0,
            flipped: false,
        };
        let mut right = Tile {
            m: right,
            id: 2,
            rot: 0,
            flipped: false,
        };
        let mut below = Tile {
            m: below,
            id: 3,
            rot: 0,
            flipped: false,
        };

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

fn solve(
    i: usize,
    width: usize,
    tiles: &mut [Tile],
    avail: &mut [bool],
    result: &mut [usize],
    fit_memo: &mut HashMap<String, bool>,
) -> bool {
    if i == tiles.len() {
        return true;
    }
    for t in 0..tiles.len() {
        if !avail[t] {
            continue;
        }
        avail[t] = false;
        for k in 0..8 {
            if k == 4 {
                flip(&mut tiles[t]);
            }
            rotate(&mut tiles[t]);
            if fits(width, tiles, t, i, result, fit_memo) {
                result[i] = t;
                if solve(i + 1, width, tiles, avail, result, fit_memo) {
                    return true;
                }
                result[i] = tiles.len();
            }
        }
        avail[t] = true;
    }
    false
}

fn corner_product(tiles: &[Tile], result: &[usize], width: usize) -> u64 {
    tiles[result[0]].id as u64
        * tiles[result[width - 1]].id as u64
        * tiles[result[result.len() - width]].id as u64
        * tiles[result[result.len() - 1]].id as u64
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let mut tiles = parse_tiles(&text);
    let width = (tiles.len() as f64).sqrt() as usize;
    let mut result = vec![tiles.len(); tiles.len()];
    let mut avail = vec![true; tiles.len()];
    let mut fit_memo = HashMap::new();
    assert!(solve(
        0,
        width,
        &mut tiles,
        &mut avail,
        &mut result,
        &mut fit_memo
    ));
    println!("{}", corner_product(&tiles, &result, width));
}
