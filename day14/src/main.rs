use regex::Regex;
use std::collections::HashMap;

fn atoi(s: &str) -> u64 {
    u64::from_str_radix(s, 10).unwrap()
}

fn btoi(s: &str) -> u64 {
    u64::from_str_radix(s, 2).unwrap()
}

fn clamp36(x: u64) -> u64 {
    x & ((!0) >> 28)
}

struct VM {
    float_bit_indices: Vec<usize>,
    mask_re: Regex,
    mem_re: Regex,
    or: u64,
    and: u64,
    mem: HashMap<u64, u64>,
}

impl VM {
    fn new() -> Self {
        let mask_re = Regex::new(r"^mask = (.+)$").unwrap();
        let mem_re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        let or = 0;
        let and = 1 /* NO */;
        let mem = HashMap::new();
        let float_bit_indices = Vec::new();
        VM { float_bit_indices, mask_re, mem_re, or, and, mem }
    }

    fn set_floating(&mut self, addr: u64, val: u64) {
        for b in 0..1 << self.float_bit_indices.len() {
            let mut faddr = addr;
            for (i, ix) in self.float_bit_indices.iter().enumerate() {
                if b & (1 << i) > 0 {
                    faddr |= 1 << ix;
                } else {
                    faddr &= !(1 << ix);
                }
            }
            self.mem.insert(faddr, val);
        }
    }

    fn exec2(&mut self, s: &str) {
        if let Some(caps) = self.mask_re.captures(s) {
            let mask = caps.get(1).unwrap().as_str();
            self.or = clamp36(btoi(&mask.replace('X', "0")));
            self.float_bit_indices = mask
                .chars()
                .rev()
                .enumerate()
                .filter(|(_, ch)| ch == &'X')
                .map(|(i, _)| i)
                .collect();
        } else if let Some(caps) = self.mem_re.captures(s) {
            let addr = atoi(caps.get(1).unwrap().as_str());
            let val = atoi(caps.get(2).unwrap().as_str());
            self.set_floating(addr | self.or, val);
        }
    }

    fn exec(&mut self, s: &str) {
        if let Some(caps) = self.mask_re.captures(s) {
            let mask = caps.get(1).unwrap().as_str();
            let or = clamp36(btoi(&mask.replace('X', "0")));
            let and = clamp36(btoi(&mask.replace('X', "1")));
            self.or = or;
            self.and = and;
        } else if let Some(caps) = self.mem_re.captures(s) {
            let addr = atoi(caps.get(1).unwrap().as_str());
            let val = atoi(caps.get(2).unwrap().as_str());
            let val = (val | self.or) & self.and;
            self.mem.insert(addr, val);
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();

    let mut vm = VM::new();
    for line in text.lines() {
        vm.exec(line);
    }
    println!("{}", vm.mem.values().sum::<u64>());

    let mut vm = VM::new();
    for line in text.lines() {
        vm.exec2(line);
    }
    println!("{}", vm.mem.values().sum::<u64>());
}
