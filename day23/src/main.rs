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

fn to_remove(v: &[usize], n: usize, from: usize) -> Vec<usize> {
    (from..from + n).map(|i| v[i % v.len()]).collect()
}

fn play(v: &mut Vec<usize>, cur_index: usize) -> usize {
    let cur = v[cur_index];
    let remove = to_remove(v, 3, cur_index + 1);
    let dest = destination(cur, &remove, v.len());
    let dest_index = v.iter().position(|x| *x == dest).unwrap();

    let mut tmp = Vec::with_capacity(v.len());
    tmp.push(dest);
    for cup in &remove {
        tmp.push(*cup);
    }
    for k in 0..v.len() - 1 {
        let i = (dest_index + k + 1) % v.len();
        if remove.contains(&v[i]) {
            // do this faster
            continue;
        }
        tmp.push(v[i]);
    }
    let new_cur_index = tmp.iter().position(|x| *x == v[cur_index]).unwrap();
    *v = tmp;
    (new_cur_index + 1) % v.len()
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();

    let mut v: Vec<_> =
        text.lines().next().unwrap().chars().map(atoi).collect();
    let mut start = 0;
    for _ in 0..100 {
        start = play(&mut v, start);
    }

    let pos1 = v.iter().position(|x| *x == 1).unwrap();
    let from1: Vec<_> =
        v.iter().cycle().skip(pos1 + 1).take(v.len() - 1).collect();
    let str1 = from1.iter().map(|x| x.to_string()).collect::<String>();
    println!("{}", str1);
}
