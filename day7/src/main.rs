use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type Reqs = HashMap<String, i32>;

struct Parser {
    req_re: Regex,
}

fn atoi(s: &str) -> i32 {
    i32::from_str_radix(s, 10).unwrap()
}

impl Parser {
    fn new() -> Self {
        let req_re = Regex::new(r"^(\d+) ((\w+ ?)+) bags?\.?$").unwrap();
        Self { req_re }
    }

    fn parse(&self, s: &str) -> (String, Reqs) {
        let mut toks = s.split(" contain ");
        let head = toks.next().unwrap();
        let head = head[..head.len() - 5].to_string();
        let tail = toks.next().unwrap();
        if tail == "no other bags." {
            return (head, Reqs::new());
        }
        let reqs = tail
            .split(", ")
            .map(|tok| {
                let caps = self.req_re.captures(tok).unwrap();
                let amt = atoi(caps.get(1).unwrap().as_str());
                let req = caps.get(2).unwrap().as_str().to_string();
                (req, amt)
            })
            .collect();
        (head, reqs)
    }
}

fn containing_bags(
    rules: &HashMap<String, Reqs>,
    seen: &HashSet<String>,
    bag: &str,
) -> HashSet<String> {
    let mut bags: HashSet<String> = rules
        .iter()
        .filter(|(_, reqs)| reqs.contains_key(bag))
        .map(|(out, _)| out.clone())
        .filter(|bag| !seen.contains(bag))
        .collect();
    let extra: HashSet<String> = bags
        .iter()
        .flat_map(|bag| containing_bags(rules, &bags, bag))
        .collect();
    bags.extend(extra.iter().cloned());
    bags
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let parser = Parser::new();
    let rules: HashMap<String, Reqs> =
        text.lines().map(|s| parser.parse(s)).collect();
    println!("{}", containing_bags(&rules, &HashSet::new(), "shiny gold").len());
}
