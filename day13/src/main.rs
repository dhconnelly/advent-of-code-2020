mod crt;

fn atoi(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap()
}

fn wait_time(from: i64, bus: i64) -> i64 {
    if from % bus == 0 {
        return 0;
    }
    let arrival = (from / bus + 1) * bus;
    arrival - from
}

fn first_arrival(from: i64, buses: &[&str]) -> (i64, i64) {
    let buses = buses.iter().copied().filter(|s| s != &"x").map(atoi);
    let mut arrivals: Vec<_> =
        buses.map(|bus| (bus, wait_time(from, bus))).collect();
    arrivals.sort_by_key(|(_, wait)| *wait);
    arrivals[0]
}

fn sequence_timestamp(buses: &[&str]) -> i64 {
    let reqs: Vec<(i64, i64)> = buses
        .iter()
        .enumerate()
        .filter(|(_, s)| s != &&"x")
        .map(|(i, s)| (i as i64, atoi(s)))
        .map(|(i, n)| ((n * 100 - i) % n, n))
        .collect();
    // for each (a, n) in reqs, we want x s.t. x = a (mod n)
    let residues: Vec<_> = reqs.iter().map(|(i, _)| *i).collect();
    let moduli: Vec<_> = reqs.iter().map(|(_, n)| *n).collect();
    let n = crt::chinese_remainder(&residues, &moduli).unwrap();
    n
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<_> = text.lines().collect();
    let earliest = atoi(lines[0]);
    let buses: Vec<_> = lines[1].split(',').collect();

    let (bus, wait) = first_arrival(earliest, &buses);
    println!("{}", bus * wait);

    println!("{}", sequence_timestamp(&buses));
}
