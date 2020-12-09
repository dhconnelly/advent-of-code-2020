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
        let mut validator = Self {
            size: cache.len(),
            sums: HashMap::new(),
            cache: cache.clone(),
        };
        for &(x, xts) in &cache {
            validator.add_to_cache(x, xts);
        }
        validator
    }

    fn add_to_cache(&mut self, x: i64, xts: Timestamp) {
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
    }

    fn is_valid(&self, x: i64, ts: Timestamp) -> bool {
        self.sums
            .get(&x)
            .map_or(false, |cache_ts| ts - cache_ts <= self.size)
    }

    fn update(&mut self, x: i64, xts: Timestamp) {
        self.cache.pop_front();
        self.add_to_cache(x, xts);
        self.cache.push_back((x, xts));
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let nums: Vec<_> = text.lines().map(atoi).collect();
    let preamble = nums.iter().copied().take(WINDOW_SIZE);
    let mut validator = WindowedValidator::new(preamble);

    let invalid = nums.iter().enumerate().skip(WINDOW_SIZE).find(|(ts, x)| {
        if validator.is_valid(**x, *ts) {
            validator.update(**x, *ts);
            false
        } else {
            true
        }
    }).unwrap().1;
    println!("{}", invalid);

    let mut sum_per_range: HashMap<(usize, usize), i64> = HashMap::new();
    let mut range_per_sum: HashMap<i64, (usize, usize)> = HashMap::new();
    sum_per_range.insert((0, 0), nums[0]);
    range_per_sum.insert(nums[0], (0, 0));
    for (i, x) in nums.iter().enumerate().skip(1) {
        if let Some(&(from, to)) = range_per_sum.get(invalid) {
            if from != to {
                let min = nums[from..=to].iter().min().unwrap();
                let max = nums[from..=to].iter().max().unwrap();
                println!("{}", min + max);
                break;
            }
        }
        for from in 0..i {
            let sum = sum_per_range.get(&(from, i-1)).unwrap_or(&nums[i-1]) + x;
            sum_per_range.insert((from, i), sum);
            range_per_sum.insert(sum, (from, i));
        }
    }
}
