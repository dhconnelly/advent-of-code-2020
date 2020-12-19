use std::collections::VecDeque;

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
        let seq1 = parse_seq(segs.next().unwrap());
        let seq2 = parse_seq(segs.next().unwrap());
        Rule::Alt(seq1, seq2)
    } else {
        Rule::Seq(parse_seq(s))
    }
}

struct Matcher {
    rules: Vec<Rule>,
}

impl Matcher {
    fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    fn match_char(&self, ch: char, s: &str, q: &mut VecDeque<usize>) -> bool {
        match s.chars().next() {
            Some(ch1) if ch == ch1 => self.matches_all(&s[1..], q),
            _ => false,
        }
    }

    fn match_seq(
        &self,
        seq: &[usize],
        s: &str,
        q: &mut VecDeque<usize>,
    ) -> bool {
        for r in seq.iter().rev() {
            q.push_front(*r);
        }
        self.matches_all(s, q)
    }

    fn matches_all(&self, s: &str, mut q: &mut VecDeque<usize>) -> bool {
        if q.is_empty() && s.is_empty() {
            return true;
        }
        if q.is_empty() || s.is_empty() {
            return false;
        }
        let first = &self.rules[q.pop_front().unwrap()];
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
        let mut q = VecDeque::new();
        q.push_back(0);
        self.matches_all(s, &mut q)
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let segs: Vec<_> = text.split("\n\n").collect();
    let mut rules_text: Vec<_> = segs[0].lines().collect();
    rules_text.sort_by_key(|s| atoi(s.split(": ").next().unwrap()));
    let rules: Vec<_> = rules_text.iter().map(|s| parse_rule(s)).collect();
    let matcher = Matcher::new(rules);
    let valid = segs[1].lines().filter(|s| matcher.matches(s)).count();
    println!("{}", valid);
}
