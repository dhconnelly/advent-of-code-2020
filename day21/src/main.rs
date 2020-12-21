use std::collections::BTreeSet;
use std::collections::HashMap;

fn read_ingredients(s: &str) -> BTreeSet<&str> {
    s.lines()
        .flat_map(|line| {
            let to = line.find(" (").unwrap_or(line.len());
            line[..to].split(" ")
        })
        .collect()
}

const SEP: &'static str = " (contains ";

fn read_possible_sources(s: &str) -> HashMap<&str, BTreeSet<&str>> {
    let mut sources: HashMap<&str, BTreeSet<&str>> = HashMap::new();
    for l in s.lines() {
        let from = l.find(SEP).unwrap_or(l.len());
        let ingreds: BTreeSet<&str> = l[..from].split(" ").collect();
        let allergens = l[from + SEP.len()..l.len() - 1].split(", ");
        for allergen in allergens {
            if let Some(existing) = sources.get(allergen) {
                let intersection =
                    existing.intersection(&ingreds).copied().collect();
                sources.insert(allergen, intersection);
            } else {
                sources.insert(allergen, ingreds.clone());
            }
        }
    }
    sources
}

fn solve<'t>(
    sources: &HashMap<&'t str, &'t str>,
    possible_sources: &HashMap<&'t str, BTreeSet<&'t str>>,
) -> Option<HashMap<&'t str, &'t str>> {
    if possible_sources.is_empty() {
        return Some(sources.clone());
    }
    for ingreds in possible_sources.values() {
        if ingreds.len() == 0 {
            return None;
        }
    }
    for (allergen, ingreds) in possible_sources.iter() {
        for ingred in ingreds {
            let mut sources = sources.clone();
            sources.insert(allergen, ingred);
            let mut possible_sources = possible_sources.clone();
            possible_sources.remove(allergen);
            for (_, ingreds) in &mut possible_sources {
                ingreds.remove(ingred);
            }
            if let Some(sources) = solve(&sources, &possible_sources) {
                return Some(sources.clone());
            }
        }
    }
    None
}

fn safe_count<'t>(
    lines: impl Iterator<Item = &'t str>,
    safe_ingreds: &BTreeSet<&'t str>,
) -> usize {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for line in lines {
        let to = line.find(" (").unwrap_or(line.len());
        for ingred in line[..to].split(" ") {
            if safe_ingreds.contains(ingred) {
                *counts.entry(ingred).or_default() += 1;
            }
        }
    }
    counts.values().sum()
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let possible_sources = read_possible_sources(&text);
    let ingreds = read_ingredients(&text);

    let sources = solve(&HashMap::new(), &possible_sources).unwrap();
    let unsafe_ingreds = sources.values().copied().collect();
    let safe_ingreds = ingreds.difference(&unsafe_ingreds).copied().collect();
    let safe = safe_count(text.lines(), &safe_ingreds);
    println!("{}", safe);

    let mut allergens: Vec<_> = sources.keys().copied().collect();
    allergens.sort();
    let unsafe_ingreds: Vec<_> = allergens
        .iter()
        .map(|allergen| sources.get(allergen).unwrap())
        .copied()
        .collect();
    println!("{}", (&unsafe_ingreds).join(","));
}
