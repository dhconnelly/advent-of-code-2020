type Grid = Vec<Vec<char>>;

struct Tile {
    id: usize,
    m: Grid,
}

fn parse_grid(s: &str) -> Grid {
    s.lines().map(|line| line.chars().collect()).collect()
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

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let tiles = parse_tiles(&text);
    println!("{}", tiles.len());
}
