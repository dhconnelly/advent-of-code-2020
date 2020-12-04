use regex::Regex;
use std::collections::HashMap;

type Entry<'t> = HashMap<&'t str, &'t str>;

fn parse_entries(s: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    for line in s.split("\n\n") {
        let mut m = HashMap::new();
        for segment in line.split_ascii_whitespace() {
            let mut toks = segment.split(':');
            let k = toks.next().unwrap();
            let v = toks.next().unwrap();
            m.insert(k, v);
        }
        entries.push(m);
    }
    entries
}

fn has_fields(e: &Entry) -> bool {
    e.len() == 8 || (e.len() == 7 && !e.contains_key("cid"))
}

fn atoi(s: &str) -> Option<i32> {
    i32::from_str_radix(s, 10).ok()
}

struct Validator {
    res: HashMap<&'static str, Regex>,
}

impl Validator {
    fn new() -> Self {
        let mut res = HashMap::new();
        let regex = |s| Regex::new(s).unwrap();
        res.insert("byr", regex(r"^\d{4}$"));
        res.insert("iyr", regex(r"^\d{4}$"));
        res.insert("eyr", regex(r"^\d{4}$"));
        res.insert("hgt", regex(r"^\d+(cm|in)$"));
        res.insert("hcl", regex(r"^#[0-9a-f]{6}$"));
        res.insert("ecl", regex(r"^(amb|blu|brn|gry|grn|hzl|oth)$"));
        res.insert("pid", regex(r"^[0-9]{9}$"));
        Self { res }
    }

    fn is_valid(&self, e: &Entry) -> bool {
        let valid_fields = e
            .iter()
            .filter(|(&k, _)| k != "cid")
            .filter(|(&k, v)| self.res[k].is_match(v))
            .filter(|(&k, v)| match k {
                "byr" => atoi(v).map_or(false, |n| n >= 1920 && n <= 2002),
                "iyr" => atoi(v).map_or(false, |n| n >= 2010 && n <= 2020),
                "eyr" => atoi(v).map_or(false, |n| n >= 2020 && n <= 2030),
                "hgt" => match &v[v.len() - 2..] {
                    "cm" => atoi(&v[..3]).map_or(false, |n| n >= 150 && n <= 193),
                    "in" => atoi(&v[..2]).map_or(false, |n| n >= 59 && n <= 76),
                    _ => unreachable!(),
                },
                _ => true,
            })
            .count();
        return has_fields(e) && valid_fields == 7;
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let entries = parse_entries(&text);
    let num_complete = entries.iter().filter(|e| has_fields(e)).count();
    println!("{}", num_complete);

    let v = Validator::new();
    let num_valid = entries.iter().filter(|e| v.is_valid(e)).count();
    println!("{}", num_valid);
}
