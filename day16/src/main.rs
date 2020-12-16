use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Rule {
    lo1: i64,
    hi1: i64,
    lo2: i64,
    hi2: i64,
}

fn atoi(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap()
}

fn parse_rules(s: &str) -> Vec<Rule> {
    let re = regex::Regex::new(r".+: (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    s.lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Rule {
                lo1: atoi(caps.get(1).unwrap().as_str()),
                hi1: atoi(caps.get(2).unwrap().as_str()),
                lo2: atoi(caps.get(3).unwrap().as_str()),
                hi2: atoi(caps.get(4).unwrap().as_str()),
            }
        })
        .collect()
}

fn parse_ticket(s: &str) -> Ticket {
    s.split(',').map(atoi).collect()
}

struct Validator {
    allowed: std::collections::HashSet<i64>,
}

type Ticket = Vec<i64>;

impl Validator {
    fn new(rules: &[Rule]) -> Self {
        let mut allowed = HashSet::new();
        for rule in rules {
            for i in rule.lo1..rule.hi1 + 1 {
                allowed.insert(i);
            }
            for i in rule.lo2..rule.hi2 + 1 {
                allowed.insert(i);
            }
        }
        Self { allowed }
    }

    fn error_rate(&self, ticket: &Ticket) -> i64 {
        ticket.iter().filter(|n| !self.allowed.contains(n)).sum()
    }

    fn total_error_rate(&self, tix: &[Ticket]) -> i64 {
        tix.iter().map(|ticket| self.error_rate(ticket)).sum()
    }

    fn valid_tickets(&self, tix: &[Ticket]) -> Vec<Ticket> {
        tix.iter()
            .filter(|ticket| self.error_rate(ticket) == 0)
            .cloned()
            .collect()
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let mut segs = text.split("\n\n");
    let rules = parse_rules(segs.next().unwrap());
    let mine =
        parse_ticket(segs.next().unwrap().lines().skip(1).next().unwrap());
    let nearby: Vec<_> = segs
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();

    let validator = Validator::new(&rules);
    println!("{}", validator.total_error_rate(&nearby));
}