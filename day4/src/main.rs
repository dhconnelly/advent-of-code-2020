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

struct Validator {
    res: HashMap<&'static str, Regex>,
}

impl Validator {
    fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("byr", Regex::new(r"^\d{4}$").unwrap());
        m.insert("iyr", Regex::new(r"^\d{4}$").unwrap());
        m.insert("eyr", Regex::new(r"^\d{4}$").unwrap());
        m.insert("hgt", Regex::new(r"^\d+(cm|in)$").unwrap());
        m.insert("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap());
        m.insert(
            "ecl",
            Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
        );
        m.insert("pid", Regex::new(r"^[0-9]{9}$").unwrap());
        m.insert("cid", Regex::new(".*").unwrap());
        Self { res: m }
    }

    fn is_valid(&self, e: &Entry) -> bool {
        let valid_fields = e
            .iter()
            .filter(|(&k, _)| k != "cid")
            .filter(|(&k, v)| self.res.get(k).unwrap().is_match(v))
            .filter(|(&k, v)| match k {
                "byr" => match i32::from_str_radix(v, 10) {
                    Ok(n) => n >= 1920 && n <= 2002,
                    Err(_) => false,
                },
                "iyr" => match i32::from_str_radix(v, 10) {
                    Ok(n) => n >= 2010 && n <= 2020,
                    Err(_) => false,
                },
                "eyr" => match i32::from_str_radix(v, 10) {
                    Ok(n) => n >= 2020 && n <= 2030,
                    Err(_) => false,
                },
                "hgt" => match &v[v.len() - 2..] {
                    "cm" => match i32::from_str_radix(&v[..3], 10) {
                        Ok(n) => n >= 150 && n <= 193,
                        Err(_) => false,
                    },
                    "in" => match i32::from_str_radix(&v[..2], 10) {
                        Ok(n) => n >= 59 && n <= 76,
                        Err(_) => false,
                    },
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
