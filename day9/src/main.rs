use std::collections::HashMap;
use std::collections::VecDeque;

fn atoi(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap()
}

const WINDOW_SIZE: usize = 25;

type Timestamp = usize;

#[derive(Debug)]
struct WindowedValidator {
    size: usize,
    sums: HashMap<i64, Timestamp>,
    cache: VecDeque<(i64, Timestamp)>,
}

impl WindowedValidator {
    fn new<I>(preamble: I) -> Self where I: Iterator<Item = i64> {
        let cache: VecDeque<_> =
            preamble.enumerate().map(|(i, x)| (x, i)).collect();
        let mut sums = HashMap::new();
        for (x, xts) in &cache {
            for (y, yts) in &cache {
                if x == y {
                    continue;
                }
                let sum = x + y;
                let ts1 = *xts.min(yts);
                match sums.get(&(x + y)) {
                    None => sums.insert(sum, ts1),
                    Some(&ts2) if ts1 > ts2 => sums.insert(sum, ts1),
                    Some(_) => continue,
                };
            }
        }
        Self { size: cache.len(), sums, cache }
    }

    fn is_valid(&self, x: i64, ts: Timestamp) -> bool {
        self.sums.get(&x).map_or(false, |cache_ts| ts - cache_ts <= self.size)
    }

    fn update(&mut self, x: i64, xts: Timestamp) {
        self.cache.pop_front();
        for &(y, yts) in &self.cache {
            if x == y {
                continue;
            }
            let sum = x + y;
            let ts1 = xts.min(yts);
            match self.sums.get(&(x + y)) {
                None => self.sums.insert(sum, ts1),
                Some(&ts2) if ts1 > ts2 => self.sums.insert(sum, ts1),
                Some(_) => continue,
            };
        }
        self.cache.push_back((x, xts));
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let nums: Vec<_> = text.lines().map(atoi).collect();
    let preamble = nums.iter().copied().take(WINDOW_SIZE);
    let mut validator = WindowedValidator::new(preamble);

    for (ts, &x) in nums.iter().enumerate().skip(WINDOW_SIZE) {
        if validator.is_valid(x, ts) {
            validator.update(x, ts);
        } else {
            println!("{}", x);
            break;
        }
    }
}
