#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instr {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instr {
    fn parse(s: &str) -> Self {
        let mut toks = s.split(' ');
        let op = toks.next().unwrap();
        let arg = atoi(toks.next().unwrap());
        match op {
            "acc" => Instr::Acc(arg),
            "jmp" => Instr::Jmp(arg),
            "nop" => Instr::Nop(arg),
            _ => unreachable!(),
        }
    }
}
        

fn atoi(s: &str) -> i32 {
    i32::from_str_radix(s, 10).unwrap()
}

fn read_program(s: &str) -> Vec<Instr> {
    s.lines().map(Instr::parse).collect()
}

struct VM {
    acc: i32,
    pc: i32,
    prog: Vec<Instr>,
}

impl VM {
    fn new(prog: Vec<Instr>) -> Self {
        VM { acc: 0, pc: 0, prog }
    }

    fn cur(&self) -> Instr {
        self.prog[self.pc as usize]
    }

    fn step(&mut self) {
        match self.cur() {
            Instr::Acc(n) => {
                self.acc += n;
                self.pc += 1;
            }
            Instr::Jmp(n) => {
                self.pc += n;
            }
            Instr::Nop(_) => {
                self.pc += 1;
            }
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let prog = read_program(&text);
    let mut vm = VM::new(prog);
    let mut seen = std::collections::HashSet::new();
    while !seen.contains(&vm.pc) {
        seen.insert(vm.pc);
        vm.step();
    }
    println!("{}", vm.acc);
}
