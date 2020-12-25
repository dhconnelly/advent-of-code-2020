fn discrete_log(g: u64, y: u64, m: u64) -> u64 {
    let mut n = 1;
    let mut res = g;
    while res != y {
        res *= g;
        res %= m;
        n += 1;
    }
    n
}

fn exp(g: u64, mut x: u64, m: u64) -> u64 {
    let mut res = 1;
    while x > 0 {
        res *= g;
        res %= m;
        x -= 1;
    }
    res
}

fn compute_secret(pk1: u64, pk2: u64, g: u64, m: u64) -> u64 {
    let x = discrete_log(g, pk1, m);
    exp(pk2, x, m)
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let mut nums = text.lines().map(|x| u64::from_str_radix(x, 10).unwrap());
    let pk1 = nums.next().unwrap();
    let pk2 = nums.next().unwrap();
    let sec = compute_secret(pk1, pk2, 7, 20201227);
    println!("{}", sec);
}
