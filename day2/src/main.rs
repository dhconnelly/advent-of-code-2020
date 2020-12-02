use regex::Regex;

struct Rule {
    min: usize,
    max: usize,
    ch: char,
}

struct Entry<'a> {
    rule: Rule,
    pw: &'a str,
}

fn parse<'a>(re: &Regex, s: &'a str) -> Entry<'a> {
    let caps = re.captures(s).unwrap();
    let rule = Rule {
        min: usize::from_str_radix(caps.get(1).unwrap().as_str(), 10).unwrap(),
        max: usize::from_str_radix(caps.get(2).unwrap().as_str(), 10).unwrap(),
        ch: caps.get(3).unwrap().as_str().chars().next().unwrap(),
    };
    let pw = caps.get(4).unwrap().as_str();
    Entry { rule, pw }
}

fn valid(entry: &Entry) -> bool {
    let count = entry.pw.chars().filter(|ch| *ch == entry.rule.ch).count();
    entry.rule.min <= count && count <= entry.rule.max
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let pat = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
    let matches = text
        .lines()
        .map(|s| parse(&pat, s))
        .filter(valid)
        .count();
    println!("{}", matches);
}
