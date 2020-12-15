fn nth(n: i64, nums: &[i64]) -> i64 {
    let mut nums = nums.iter().copied();
    let mut last_seen = std::collections::HashMap::new();
    let mut next: i64 = 0;
    for i in 1..n {
        let next0 = match nums.next() {
            Some(num) => num,
            None => next,
        };
        let next1 = match last_seen.get(&next0) {
            Some(ts) => i - ts,
            None => 0,
        };
        last_seen.insert(next0, i);
        next = next1;
    }
    next
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let nums: Vec<_> = text
        .split(',')
        .map(|s| i64::from_str_radix(s.trim(), 10).unwrap())
        .collect();
    println!("{}", nth(2020, &nums));
    println!("{}", nth(30000000, &nums));
}
