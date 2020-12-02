use regex::Regex;

struct Rule {
    min: usize,
    max: usize,
    ch: u8,
}

struct Entry<'a> {
    rule: Rule,
    pw: &'a [u8],
}

fn parse<'a>(re: &Regex, s: &'a str) -> Entry<'a> {
    let caps = re.captures(s).unwrap();
    let rule = Rule {
        min: usize::from_str_radix(caps.get(1).unwrap().as_str(), 10).unwrap(),
        max: usize::from_str_radix(caps.get(2).unwrap().as_str(), 10).unwrap(),
        ch: caps.get(3).unwrap().as_str().bytes().next().unwrap(),
    };
    let pw = caps.get(4).unwrap().as_str().as_bytes();
    Entry { rule, pw }
}

fn validate1(entry: &Entry) -> bool {
    let count = entry.pw.iter().filter(|ch| **ch == entry.rule.ch).count();
    entry.rule.min <= count && count <= entry.rule.max
}

fn validate2(entry: &Entry) -> bool {
    let ch1 = entry.pw[entry.rule.min-1];
    let ch2 = entry.pw[entry.rule.max-1];
    let ch = entry.rule.ch;
    (ch1 == ch && ch2 != ch) || (ch1 != ch && ch2 == ch)
}

fn count_valid1(db: &[Entry]) -> usize {
    db.iter().filter(|e| validate1(e)).count()
}

fn count_valid2(db: &[Entry]) -> usize {
    db.iter().filter(|e| validate2(e)).count()
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let pat = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
    let db: Vec<_> = text.lines().map(|s| parse(&pat, s)).collect();
    println!("{}", count_valid1(&db));
    println!("{}", count_valid2(&db));
}
