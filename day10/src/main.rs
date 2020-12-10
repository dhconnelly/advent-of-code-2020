use std::collections::HashMap;

fn atoi(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap()
}

fn differences(chain: &[i64], of: i64) -> usize {
    let mut i = 0;
    let mut last = chain[0];
    for x in chain.iter().skip(1) {
        if x - last == of {
            i += 1;
        }
        last = *x;
    }
    i
}

// how many valid chains include |from| and any nums greater than |from|?
fn count(nums: &[i64], from: usize, memo: &mut HashMap<usize, i64>) -> i64 {
    if let Some(n) = memo.get(&from) {
        return *n;
    } else if from >= nums.len() {
        return 0;
    }

    let cur = nums[from];
    let (a, b, c) = (from + 1, from + 2, from + 3);
    let mut sum = count(nums, a, memo); // 1 X X
    if b < nums.len() && nums[b] - cur <= 3 {
        sum += count(nums, b, memo); // 0 1 X
    }
    if c < nums.len() && nums[c] - cur <= 3 {
        sum += count(nums, c, memo); // 0 0 1
    }

    memo.insert(from, sum);
    sum
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let mut nums: Vec<_> = text.lines().map(atoi).collect();
    nums.push(0);
    nums.sort();
    nums.push(nums[nums.len() - 1] + 3);
    println!("{}", differences(&nums, 1) * differences(&nums, 3));
    let mut memo = HashMap::new();
    memo.insert(nums.len() - 2, 1);
    memo.insert(nums.len() - 1, 1);
    println!("{}", count(&nums, 0, &mut memo));
}
