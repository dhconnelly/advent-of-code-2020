fn atoi(s: &str) -> i32 {
    i32::from_str_radix(s, 10).unwrap()
}

fn wait_time(from: i32, bus: i32) -> i32 {
    if from % bus == 0 {
        return 0;
    }
    let arrival = (from / bus + 1) * bus;
    arrival - from
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let mut lines = text.lines();
    let earliest = atoi(lines.next().unwrap());
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|s| s != &"x")
        .map(atoi);
    let mut arrivals: Vec<_> =
        buses.map(|bus| (bus, wait_time(earliest, bus))).collect();
    arrivals.sort_by_key(|(_, wait)| *wait);
    let (bus, wait) = arrivals[0];
    println!("{}", bus * wait);
}
