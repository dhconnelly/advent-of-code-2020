use std::collections::HashMap;

fn atoi(s: &str) -> usize {
    usize::from_str_radix(s, 10).unwrap()
}

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Seq(Vec<usize>),
    Alt(Vec<usize>, Vec<usize>),
}

fn parse_seq(s: &str) -> Vec<usize> {
    s.split(" ").map(atoi).collect()
}

fn parse_rule(s: &str) -> Rule {
    if s.contains('"') {
        Rule::Char(s.chars().nth(s.len() - 2).unwrap())
    } else if s.contains('|') {
        let mut segs = s.split(" | ");
        let seq1 = parse_seq(segs.next().unwrap());
        let seq2 = parse_seq(segs.next().unwrap());
        Rule::Alt(seq1, seq2)
    } else {
        Rule::Seq(parse_seq(s))
    }
}

struct Matcher {
    rules: HashMap<usize, Rule>,
}

impl Matcher {
    fn new(rules: HashMap<usize, Rule>) -> Self {
        Self { rules }
    }

    fn match_char(&self, ch: char, s: &str, q: &mut Vec<usize>) -> bool {
        match s.chars().next() {
            Some(ch1) if ch == ch1 => self.matches_all(&s[1..], q),
            _ => false,
        }
    }

    fn match_seq(&self, seq: &[usize], s: &str, q: &mut Vec<usize>) -> bool {
        seq.iter().rev().for_each(|r| q.push(*r));
        self.matches_all(s, q)
    }

    fn matches_all(&self, s: &str, mut q: &mut Vec<usize>) -> bool {
        if q.is_empty() && s.is_empty() {
            return true;
        }
        if q.is_empty() || s.is_empty() {
            return false;
        }
        let first = &self.rules[&q.pop().unwrap()];
        match first {
            Rule::Char(ch) => self.match_char(*ch, s, &mut q),
            Rule::Seq(seq) => self.match_seq(&seq, s, &mut q),
            Rule::Alt(seq1, seq2) => {
                self.match_seq(&seq1, s, &mut q.clone())
                    || self.match_seq(&seq2, s, &mut q.clone())
            }
        }
    }

    fn matches(&self, s: &str) -> bool {
        self.matches_all(s, &mut vec![0])
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let segs: Vec<_> = text.split("\n\n").collect();

    let rules: HashMap<usize, Rule> = segs[0]
        .lines()
        .map(|s| {
            let mut toks = s.split(": ");
            let n = atoi(toks.next().unwrap());
            let rule = parse_rule(toks.next().unwrap());
            (n, rule)
        })
        .collect();

    let matcher = Matcher::new(rules.clone());
    let valid = segs[1].lines().filter(|s| matcher.matches(s)).count();
    println!("{}", valid);

    let mut rules2 = rules.clone();
    rules2.insert(8, Rule::Alt(vec![42], vec![42, 8]));
    rules2.insert(11, Rule::Alt(vec![42, 31], vec![42, 11, 31]));
    let matcher = Matcher::new(rules2);
    let valid = segs[1].lines().filter(|s| matcher.matches(s)).count();
    println!("{}", valid);
}
