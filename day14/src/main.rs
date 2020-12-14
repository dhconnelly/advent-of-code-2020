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
        VM { mask_re, mem_re, or, and, mem }
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
}
