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
    let max = text.lines().map(parse_seat).map(seat_id).max().unwrap();
    println!("{}", max);
}
