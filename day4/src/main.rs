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

fn is_valid(e: &Entry) -> bool {
    e.len() == 8 || (e.len() == 7 && !e.contains_key("cid"))
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let entries = parse_entries(&text);
    let num_valid = entries.iter().filter(|e| is_valid(e)).count();
    println!("{}", num_valid);
}
