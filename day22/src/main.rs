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
    //println!("Round");
    //println!("Player 1's deck: {:?}", p1);
    //println!("Player 2's deck: {:?}", p2);
    let f1 = p1.pop_front().unwrap();
    //println!("Player 1 plays {}", f1);
    let f2 = p2.pop_front().unwrap();
    //println!("Player 2 plays {}", f2);
    match winner(p1, p2, f1, f2) {
        Winner::Player1 => {
            //println!("Player 1 wins the round!");
            p1.push_back(f1);
            p1.push_back(f2);
        }
        Winner::Player2 => {
            //println!("Player 2 wins the round!");
            p2.push_back(f2);
            p2.push_back(f1);
        }
    }
}

fn score(d: &Deck) -> u64 {
    d.iter().rev().zip(1..).map(|(card, n)| card * n).sum()
}

fn play(mut p1: &mut Deck, mut p2: &mut Deck) -> u64 {
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
        //println!("Winning deck: {:?}", p1);
        score(&p1)
    } else {
        //println!("Winning deck: {:?}", p2);
        score(&p2)
    }
}

fn deck_string(d: &Deck) -> String {
    d.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn key(p1: &Deck, p2: &Deck) -> String {
    [deck_string(p1), deck_string(p2)].join("|")
}

enum Winner {
    Player1,
    Player2,
}

fn play_recursive(mut p1: &mut Deck, mut p2: &mut Deck) -> (Winner, u64) {
    let mut seen = std::collections::HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        let k = key(p1, p2);
        if seen.contains(&k) {
            return (Winner::Player1, score(&p1));
        }
        seen.insert(k);
        play_round(&mut p1, &mut p2, |p1, p2, f1, f2| {
            if p1.len() as u64 >= f1 && p2.len() as u64 >= f2 {
                play_recursive(&mut p1.clone(), &mut p2.clone()).0
            } else if f1 > f2 {
                Winner::Player1
            } else {
                Winner::Player2
            }
        });
    }
    if !p1.is_empty() {
        //println!("Winning deck: {:?}", p1);
        (Winner::Player1, score(&p1))
    } else {
        //println!("Winning deck: {:?}", p2);
        (Winner::Player2, score(&p2))
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let (p1, p2) = read_decks(&text);
    println!("{}", play(&mut p1.clone(), &mut p2.clone()));
    println!("{}", play_recursive(&mut p1.clone(), &mut p2.clone()).1);
}
