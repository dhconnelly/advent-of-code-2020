use std::collections::HashMap;

type Pt2 = (i32, i32);

const DIRS: [Pt2; 6] = [
    (-1, 0),  // west
    (-1, -1), // northwest
    (0, -1),  // northeast
    (1, 0),   // east
    (1, 1),   // southeast
    (0, 1),   // southwest
];

fn add(p: &Pt2, q: &Pt2) -> Pt2 {
    (p.0 + q.0, p.1 + q.1)
}

fn nbrs(p: Pt2) -> impl Iterator<Item = Pt2> {
    (0..DIRS.len()).map(move |i| add(&p, &DIRS[i]))
}

fn parse_path(s: &str) -> Vec<Pt2> {
    let mut chs = s.chars();
    let mut path = vec![];
    while let Some(ch) = chs.next() {
        match ch {
            'e' => path.push(DIRS[3]),
            'w' => path.push(DIRS[0]),
            'n' => match chs.next() {
                Some('e') => path.push(DIRS[2]),
                Some('w') => path.push(DIRS[1]),
                _ => unreachable!(),
            },
            's' => match chs.next() {
                Some('e') => path.push(DIRS[4]),
                Some('w') => path.push(DIRS[5]),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    path
}

fn walk_path(path: Vec<Pt2>) -> Pt2 {
    let mut p = (0, 0);
    for q in path {
        p = add(&p, &q);
    }
    p
}

fn flip_tiles(ps: impl Iterator<Item = Pt2>) -> HashMap<Pt2, bool> {
    let mut m: HashMap<Pt2, bool> = HashMap::new();
    for p in ps {
        m.entry(p).and_modify(|v| *v = !*v).or_insert(true);
    }
    m
}

fn next_state(
    m: &HashMap<Pt2, bool>,
    nbrs: impl Iterator<Item = Pt2>,
    p: &Pt2,
) -> bool {
    let v = *m.get(p).unwrap_or(&false);
    let n = nbrs.filter(|q| *m.get(q).unwrap_or(&false)).count();
    match (v, n) {
        (true, n) if n == 0 || n > 2 => false,
        (false, 2) => true,
        _ => v,
    }
}

fn step(m1: &mut HashMap<Pt2, bool>) -> HashMap<Pt2, bool> {
    let mut m2 = HashMap::new();
    for p in m1.keys() {
        let qs: Vec<_> = nbrs(*p).collect();
        m2.insert(*p, next_state(&m1, qs.iter().copied(), p));
        for q in qs.iter().filter(|q| !m1.contains_key(q)) {
            m2.insert(*q, next_state(&m1, nbrs(*q), &q));
        }
    }
    m2
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let paths = text.lines().map(parse_path);
    let pts = paths.map(walk_path);
    let mut m = flip_tiles(pts);
    println!("{}", m.values().filter(|x| **x).count());

    for _ in 1..=100 {
        m = step(&mut m);
    }
    println!("{}", m.values().filter(|x| **x).count());
}
