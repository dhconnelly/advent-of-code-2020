#[derive(Debug)]
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
    s.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        Rule {
            lo1: atoi(caps.get(1).unwrap().as_str()),
            hi1: atoi(caps.get(2).unwrap().as_str()),
            lo2: atoi(caps.get(3).unwrap().as_str()),
            hi2: atoi(caps.get(4).unwrap().as_str()),
        }
    }).collect()
}

fn error_rate(rules: &[Rule], tix: &[&str]) -> i64 {
    let mut allowed = std::collections::HashSet::new();
    for rule in rules {
        for i in rule.lo1..rule.hi1+1 {
            allowed.insert(i);
        }
        for i in rule.lo2..rule.hi2+1 {
            allowed.insert(i);
        }
    }
    let mut rate = 0;
    for ticket in tix {
        for val in ticket.split(',').map(atoi) {
            if !allowed.contains(&val) {
                rate += val;
            }
        }
    }
    rate
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let mut segs = text.split("\n\n");
    let rules = parse_rules(segs.next().unwrap());
    segs.next().unwrap(); // ignore my ticket
    let nearby: Vec<_> = segs.next().unwrap().lines().skip(1).collect();
    println!("{}", error_rate(&rules, &nearby));
}
