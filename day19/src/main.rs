fn atoi(s: &str) -> usize {
    usize::from_str_radix(s, 10).unwrap()
}

#[derive(Debug)]
enum Rule {
    Char(char),
    Seq(Vec<usize>),
    Alt(Vec<usize>, Vec<usize>),
}

fn parse_seq(s: &str) -> Vec<usize> {
    s.split(" ").map(atoi).collect()
}

fn parse_rule(s: &str) -> Rule {
    let mut toks = s.split(": ");
    toks.next().unwrap();
    let s = toks.next().unwrap();
    if s.contains('"') {
        Rule::Char(s.chars().nth(s.len() - 2).unwrap())
    } else if s.contains('|') {
        let mut segs = s.split(" | ");
        let alt1 = parse_seq(segs.next().unwrap());
        let alt2 = parse_seq(segs.next().unwrap());
        Rule::Alt(alt1, alt2)
    } else {
        Rule::Seq(parse_seq(s))
    }
}

struct Matcher {}

impl Matcher {
    fn new(rules: &[Rule]) -> Self {
        Self {}
    }

    fn matches_all(&self, s: &str) -> bool {
        false
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let segs: Vec<_> = text.split("\n\n").collect();
    let mut rules_text: Vec<_> = segs[0].lines().collect();
    rules_text.sort_by_key(|s| atoi(s.split(": ").next().unwrap()));
    let rules: Vec<_> = rules_text.iter().map(|s| parse_rule(s)).collect();
    for (i, rule) in rules.iter().enumerate() {
        println!("rule[{}] = {:?}", i, rule);
    }
    let matcher = Matcher::new(&rules);
    let valid = segs[1].lines().filter(|s| matcher.matches_all(s)).count();
    println!("{}", valid);
}
