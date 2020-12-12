struct Instr {
    cmd: char,
    amt: i32,
}

fn parse(s: &str) -> Instr {
    let cmd = s.chars().next().unwrap();
    let amt = i32::from_str_radix(&s[1..], 10).unwrap();
    Instr { cmd, amt }
}

enum Dir { North, East, South, West }

impl Dir {
    fn parse(ch: char) -> Self {
        match ch {
            'N' => Self::North,
            'S' => Self::South,
            'E' => Self::East,
            'W' => Self::West,
            _ => unreachable!(),
        }
    }

    fn rotate(&self, dir: char) -> Self {
        match (dir, self) {
            ('R', Self::North) => Self::East,
            ('R', Self::East )=> Self::South,
            ('R', Self::South) => Self::West,
            ('R', Self::West )=> Self::North,
            ('L', Self::North) => Self::West,
            ('L', Self::East )=> Self::North,
            ('L', Self::South) => Self::East,
            ('L', Self::West )=> Self::South,
            _ => unreachable!(),
        }
    }

    fn slope(&self) -> Pt2 {
        match self {
            Self::North => Pt2(0, 1),
            Self::East => Pt2(1, 0),
            Self::South => Pt2(0, -1),
            Self::West => Pt2(-1, 0),
        }
    }
}

struct Pt2(i32, i32);

impl Pt2 {
    fn scale(&self, amt: i32) -> Pt2 {
        Pt2(self.0 * amt, self.1 * amt)
    }

    fn add(&self, pt: &Self) -> Self {
        Pt2(self.0 + pt.0, self.1 + pt.1)
    }

    fn norm(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

struct Ship {
    dir: Dir,
    loc: Pt2,
}

impl Ship {
    fn new() -> Self {
        Ship { dir: Dir::East, loc: Pt2(0, 0) }
    }

    fn rotate(&mut self, dir: char, amt: i32) {
        assert!(amt % 90 == 0);
        let n = amt / 90;
        for _ in 0..n {
            self.dir = self.dir.rotate(dir);
        }
    }

    fn advance(&mut self, amt: i32) {
        let dir = self.dir.slope();
        let vec = dir.scale(amt);
        self.loc = self.loc.add(&vec);
    }

    fn shift(&mut self, dir: char, amt: i32) {
        let dir = Dir::parse(dir).slope();
        let vec = dir.scale(amt);
        self.loc = self.loc.add(&vec);
    }

    fn exec(&mut self, i: &Instr) {
        match i.cmd {
            'N' | 'E' | 'S' | 'W' => self.shift(i.cmd, i.amt),
            'R' | 'L' => self.rotate(i.cmd, i.amt),
            'F' => self.advance(i.amt),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let instrs = text.lines().map(parse);
    let mut ship = Ship::new();
    instrs.for_each(|i| ship.exec(&i));
    println!("{}", ship.loc.norm());
}
