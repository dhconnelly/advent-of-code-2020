fn atoi(ch: char) -> usize {
    ch as usize - '0' as usize
}

fn dec(x: usize, m: usize) -> usize {
    if x == 1 {
        m
    } else {
        x - 1
    }
}

fn destination(cur: usize, not: &[usize], m: usize) -> usize {
    let mut dest = dec(cur, m);
    while not.contains(&dest) {
        dest = dec(dest, m);
    }
    dest
}

fn play(a: &mut Vec<usize>, cur: usize) -> usize {
    let r1 = a[cur - 1];
    let r2 = a[r1 - 1];
    let r3 = a[r2 - 1];
    let dst = destination(cur, &[r1, r2, r3], a.len());
    let prev_next = a[dst - 1];
    a[cur - 1] = a[r3 - 1];
    a[dst - 1] = r1;
    a[r3 - 1] = prev_next;
    a[cur - 1]
}

fn to_successor_list(v: &[usize]) -> Vec<usize> {
    let mut a = vec![0; v.len()];
    for i in 0..v.len() {
        a[v[i] - 1] = v[(i + 1) % v.len()];
    }
    a
}

fn play_n(v: &[usize], n: usize) -> Vec<usize> {
    let mut a = to_successor_list(v);
    let mut cur = v[0];
    for _ in 0..n {
        cur = play(&mut a, cur);
    }
    a
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let v: Vec<_> = text.lines().next().unwrap().chars().map(atoi).collect();

    let a1 = play_n(&v, 100);
    let mut s = String::new();
    let mut last = 1;
    while s.len() < v.len() - 1 {
        s.push_str(&a1[last - 1].to_string());
        last = a1[last - 1];
    }
    println!("{}", s);

    let a2 = play_n(
        &v.iter()
            .copied()
            .chain(v.len() + 1..=1000000)
            .collect::<Vec<_>>(),
        10000000,
    );
    println!("{}", a2[0] * a2[a2[0] - 1]);
}
