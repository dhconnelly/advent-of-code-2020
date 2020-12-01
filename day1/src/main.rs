fn sum2(nums: &[i32]) -> Option<i32> {
    let mut seen = std::collections::HashSet::new();
    for num in nums {
        let want = 2020 - num;
        if seen.contains(&want) {
            return Some(want * num);
        }
        seen.insert(num);
    }
    None
}

fn sum3(nums: &[i32]) -> Option<i32> {
    let mut seen = std::collections::HashMap::<i32, i32>::new();
    for x in nums {
        for y in nums {
            seen.insert(x+y, x*y);
        }
    }
    for z in nums {
        if let Some(prod) = seen.get(&(2020 - z)) {
            return Some(prod * z);
        }
    }
    None
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let parse = |s| i32::from_str_radix(s, 10).unwrap();
    let nums: Vec<i32> = text.lines().map(parse).collect();
    println!("{}", sum2(&nums).unwrap());
    println!("{}", sum3(&nums).unwrap());
}
