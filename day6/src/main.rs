type Person = std::collections::HashSet<char>;
type Group = Vec<Person>;

fn read_groups(s: &str) -> Vec<Group> {
    s.split("\n\n")
        .map(|g| {
            g.lines()
                .map(|p| p.chars().collect::<Person>())
                .collect::<Group>()
        })
        .collect::<Vec<Group>>()
}

fn full_person() -> Person {
    "abcdefghijklmnopqrstuvwxyz".chars().collect()
}

fn process_groups<F>(gs: &[Group], acc: Person, f: F) -> usize
where
    F: Fn(&Person, &Person) -> Person,
{
    gs.iter()
        .map(|g| g.iter().fold(acc.clone(), |a, p| f(&a, p)))
        .map(|g| g.len())
        .sum()
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let groups = read_groups(&text);

    let any_yes = process_groups(&groups, Person::new(), |acc, p| {
        acc.union(p).cloned().collect()
    });
    println!("{}", any_yes);

    let all_yes = process_groups(&groups, full_person(), |acc, p| {
        acc.intersection(p).cloned().collect()
    });
    println!("{}", all_yes);
}
