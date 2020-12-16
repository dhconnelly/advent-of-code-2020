use std::collections::HashMap;
use std::collections::HashSet;

type Ticket = Vec<i64>;

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    lo1: i64,
    hi1: i64,
    lo2: i64,
    hi2: i64,
}

impl Rule {
    fn is_valid(&self, val: i64) -> bool {
        self.lo1 <= val && val <= self.hi1 || self.lo2 <= val && val <= self.hi2
    }
}

fn atoi(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap()
}

fn parse_rules(s: &str) -> Vec<Rule> {
    let re = regex::Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    s.lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Rule {
                name: caps.get(1).unwrap().as_str().to_string(),
                lo1: atoi(caps.get(2).unwrap().as_str()),
                hi1: atoi(caps.get(3).unwrap().as_str()),
                lo2: atoi(caps.get(4).unwrap().as_str()),
                hi2: atoi(caps.get(5).unwrap().as_str()),
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

fn can_fill(rule: &Rule, idx: usize, tix: &[Ticket]) -> bool {
    for ticket in tix {
        if !rule.is_valid(ticket[idx]) {
            return false;
        }
    }
    true
}

fn key(bools: &[bool]) -> usize {
    let mut key = 0;
    for (i, b) in bools.iter().enumerate() {
        if *b {
            key |= 1 << i;
        }
    }
    key
}

fn determine_fields_bt(
    rules: &[Rule],
    avail: &mut [bool],
    from_idx: usize,
    tix: &[Ticket],
    memo: &mut HashSet<(usize, usize)>,
    rule_indices: &mut [usize],
) -> bool {
    if from_idx == rules.len() {
        return true;
    }
    let k = key(avail);
    if memo.contains(&(k, from_idx)) {
        return false;
    }
    for (i, rule) in rules.iter().enumerate() {
        if avail[i] && can_fill(rule, from_idx, tix) {
            rule_indices[from_idx] = i;
            avail[i] = false;
            if determine_fields_bt(
                rules,
                avail,
                from_idx + 1,
                tix,
                memo,
                rule_indices,
            ) {
                return true;
            } else {
                memo.insert((k, from_idx));
            }
            avail[i] = true;
        }
    }
    false
}

fn determine_fields(rules: &[Rule], tix: &[Ticket]) -> HashMap<String, usize> {
    let mut rule_indices = vec![0; rules.len()];
    assert!(determine_fields_bt(
        rules,
        &mut vec![true; rules.len()],
        0,
        tix,
        &mut HashSet::new(),
        &mut rule_indices
    ));
    rule_indices
        .iter()
        .enumerate()
        .map(|(ticket_idx, rule_idx)| {
            (rules[*rule_idx].name.clone(), ticket_idx)
        })
        .collect()
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

    let mut valid = validator.valid_tickets(&nearby);
    valid.push(mine.clone());
    let fields = determine_fields(&rules, &valid);
    let departure: i64 = fields
        .iter()
        .filter_map(|(name, idx)| match name.find("departure") {
            Some(0) => Some(mine[*idx]),
            _ => None,
        })
        .product();
    println!("{}", departure);
}
