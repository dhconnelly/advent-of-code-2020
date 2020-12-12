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

#[derive(Clone, Copy)]
struct Pt2(i32, i32);

impl Pt2 {
    fn scale(&self, amt: i32) -> Self {
        Self(self.0 * amt, self.1 * amt)
    }

    fn add(&self, pt: &Self) -> Self {
        Self(self.0 + pt.0, self.1 + pt.1)
    }

    fn sub(&self, pt: &Self) -> Self {
        Self(self.0 - pt.0, self.1 - pt.1)
    }

    fn norm(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }

    fn rotate(&self) -> Self {
        let (x, y) = (self.1.abs(), self.0.abs());
        let (xs, ys) = match (self.0.signum(), self.1.signum()) {
            (-1, -1) => (-1, 1),
            (-1,  0) => (0, 1),
            (-1,  1) => (1, 1),
            ( 0, -1) => (-1, 0),
            ( 0,  1) => (1, 0),
            ( 1, -1) => (-1, -1),
            ( 1,  0) => (0, -1),
            ( 1,  1) => (1, -1),
            _ => unreachable!(),
        };
        Self(x * xs, y * ys)
    }

    fn rotate_by(&self, dir: char, amt: i32) -> Self {
        assert!(amt % 90 == 0);
        let n = amt / 90;
        let mut pt = *self;
        for _ in 0..n {
            match dir {
                'R' => pt = pt.rotate(),
                'L' => pt = pt.rotate().rotate().rotate(),
                _ => unreachable!(),
            }
        }
        pt
    }
}

struct WaypointShip {
    waypt: Pt2,
    loc: Pt2,
}

impl WaypointShip {
    fn new() -> Self {
        Self { waypt: Pt2(10, 1), loc: Pt2(0, 0) }
    }

    fn rotate(&mut self, dir: char, amt: i32) {
        let vec = self.waypt.sub(&self.loc);
        let vec = vec.rotate_by(dir, amt);
        self.waypt = vec.add(&self.loc);
    }

    fn advance(&mut self, amt: i32) {
        let vec = self.waypt.sub(&self.loc).scale(amt);
        self.loc = self.loc.add(&vec);
        self.waypt = self.waypt.add(&vec);
    }

    fn shift(&mut self, dir: char, amt: i32) {
        let dir = Dir::parse(dir).slope();
        let vec = dir.scale(amt);
        self.waypt = self.waypt.add(&vec);
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

struct Ship {
    dir: Dir,
    loc: Pt2,
}

impl Ship {
    fn new() -> Self {
        Self { dir: Dir::East, loc: Pt2(0, 0) }
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
    let instrs: Vec<_> = text.lines().map(parse).collect();

    let mut ship = Ship::new();
    instrs.iter().for_each(|i| ship.exec(i));
    println!("{}", ship.loc.norm());

    let mut ship2 = WaypointShip::new();
    instrs.iter().for_each(|i| ship2.exec(i));
    println!("{}", ship2.loc.norm());
}
