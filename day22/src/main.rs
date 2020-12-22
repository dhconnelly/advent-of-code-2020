use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn atoi(s: &str) -> u64 {
    u64::from_str_radix(s, 10).unwrap()
}

type Deck = VecDeque<u64>;

fn read_decks(s: &str) -> (Deck, Deck) {
    let mut segs = s
        .split("\n\n")
        .map(|seg| seg.lines().skip(1).map(atoi).collect::<Deck>());
    let p1 = segs.next().unwrap();
    let p2 = segs.next().unwrap();
    (p1, p2)
}

fn play_round(
    p1: &mut Deck,
    p2: &mut Deck,
    mut winner: impl FnMut(&Deck, &Deck, u64, u64) -> Winner,
) {
    let f1 = p1.pop_front().unwrap();
    let f2 = p2.pop_front().unwrap();
    match winner(p1, p2, f1, f2) {
        Winner::Player1 => {
            p1.push_back(f1);
            p1.push_back(f2);
        }
        Winner::Player2 => {
            p2.push_back(f2);
            p2.push_back(f1);
        }
    }
}

fn score(d1: &Deck, d2: &Deck) -> u64 {
    let d = if d1.is_empty() { d2 } else { d1 };
    d.iter().rev().zip(1..).map(|(card, n)| card * n).sum()
}

fn play(mut p1: &mut Deck, mut p2: &mut Deck) -> Winner {
    while !p1.is_empty() && !p2.is_empty() {
        play_round(&mut p1, &mut p2, |_, _, f1, f2| {
            if f1 > f2 {
                Winner::Player1
            } else {
                Winner::Player2
            }
        });
    }
    if !p1.is_empty() {
        Winner::Player1
    } else {
        Winner::Player2
    }
}

fn key(p1: &Deck, p2: &Deck) -> String {
    let mut s = String::new();
    for card in p1 {
        s.push_str(&card.to_string());
        s.push(',');
    }
    s.push('|');
    for card in p2 {
        s.push_str(&card.to_string());
        s.push(',');
    }
    s
}

#[derive(Clone, Copy)]
enum Winner {
    Player1,
    Player2,
}

fn play_recursive(
    mut p1: &mut Deck,
    mut p2: &mut Deck,
    //mut memo: &mut HashMap<String, Winner>,
) -> Winner {
    //println!("{:?} {:?}", p1, p2);
    let mut seen = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        let k = key(p1, p2);
        if seen.contains(&k) {
            return Winner::Player1;
        }
        seen.insert(k);
        play_round(&mut p1, &mut p2, |p1, p2, f1, f2| {
            if p1.len() as u64 >= f1 && p2.len() as u64 >= f2 {
                //let k1 = key(p1, p2);
                //if let Some(result) = memo.get(&k1) {
                ////println!("cache hit: {}", k1);
                //*result
                //} else {
                let result = play_recursive(
                    &mut p1
                        .clone()
                        .iter()
                        .take(f1 as usize)
                        .copied()
                        .collect(),
                    &mut p2
                        .clone()
                        .iter()
                        .take(f2 as usize)
                        .copied()
                        .collect(), //&mut memo,
                );
                //memo.insert(k1, result);
                result
            //}
            } else if f1 > f2 {
                Winner::Player1
            } else {
                Winner::Player2
            }
        });
    }
    if !p1.is_empty() {
        Winner::Player1
    } else {
        Winner::Player2
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let (deck1, deck2) = read_decks(&text);

    //let (mut p1, mut p2) = (deck1.clone(), deck2.clone());
    //play(&mut p1, &mut p2);
    //println!("{}", score(&p1, &p2));

    let (mut p1, mut p2) = (deck1.clone(), deck2.clone());
    play_recursive(&mut p1, &mut p2 /*&mut HashMap::new()*/);
    println!("{}", score(&p1, &p2));
}
