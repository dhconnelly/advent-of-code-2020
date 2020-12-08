fn atoi(s: &str) -> i32 {
    i32::from_str_radix(s, 10).unwrap()
}

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

fn read_program(s: &str) -> Vec<Instr> {
    s.lines().map(Instr::parse).collect()
}

struct VM<'t> {
    acc: i32,
    pc: i32,
    prog: &'t [Instr],
}

impl<'t> VM<'t> {
    fn new(prog: &'t [Instr]) -> Self {
        VM { acc: 0, pc: 0, prog }
    }

    fn cur(&self) -> &Instr {
        &self.prog[self.pc as usize]
    }

    fn step(&mut self) {
        match *self.cur() {
            Instr::Acc(n) => {
                self.acc += n;
                self.pc += 1;
            }
            Instr::Jmp(n) => self.pc += n,
            Instr::Nop(_) => self.pc += 1,
        }
    }

    fn done(&self) -> bool {
        self.pc == (self.prog.len() as i32)
    }

    fn run_until_loop(&mut self) {
        let mut seen = vec![false; self.prog.len()];
        while !self.done() && !seen[self.pc as usize] {
            seen[self.pc as usize] = true;
            self.step();
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let prog = read_program(&text);

    let mut vm = VM::new(&prog);
    vm.run_until_loop();
    println!("{}", vm.acc);

    let mut prog = prog;
    for i in 0..prog.len() {
        let prev = prog[i];
        prog[i] = match prev {
            Instr::Acc(_) => prev,
            Instr::Jmp(n) => Instr::Nop(n),
            Instr::Nop(n) => Instr::Jmp(n),
        };
        let mut vm = VM::new(&prog);
        vm.run_until_loop();
        if vm.done() {
            println!("{}", vm.acc);
            return;
        }
        prog[i] = prev;
    }
    unreachable!();
}
