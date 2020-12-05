type Seat = (i32, i32);

fn parse_seat(s: &str) -> Seat {
    let btoi = |s| i32::from_str_radix(s, 2).unwrap();
    let rowb = &s[..7].replace('B', "1").replace('F', "0");
    let colb = &s[7..].replace('R', "1").replace('L', "0");
    (btoi(rowb), btoi(colb))
}

fn seat_id((row, col): Seat) -> i32 {
    row * 8 + col
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let mut ids: Vec<_> = text.lines().map(parse_seat).map(seat_id).collect();
    ids.sort();

    let max = ids[ids.len() - 1];
    println!("{}", max);

    let min = ids[0];
    let (i, _) = ids
        .iter()
        .enumerate()
        .find(|(i, &id)| min + (*i as i32) != id)
        .unwrap();
    println!("{:?}", i as i32 + min);
}
