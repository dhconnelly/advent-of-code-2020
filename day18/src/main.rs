#[derive(Debug)]
enum Expr {
    Num(i64),
    BinOp(char, Vec<Expr>),
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_op(ch: char) -> bool {
    ch == '+' || ch == '*'
}

struct Parser {
    src: String,
    text: Vec<char>,
    cur: usize,
}

impl Parser {
    fn new(s: &str) -> Parser {
        Parser {
            src: s.replace(' ', ""),
            text: s.replace(' ', "").chars().collect(),
            cur: 0,
        }
    }

    fn at_end(&self) -> bool {
        assert!(self.cur <= self.text.len());
        self.cur == self.text.len()
    }

    fn peek(&self) -> char {
        self.text[self.cur]
    }

    fn next(&mut self) -> char {
        let ch = self.peek();
        self.cur += 1;
        ch
    }

    fn eat(&mut self, ch: char) {
        let got = self.peek();
        assert!(got == ch, format!("want {}, got {}", ch, got));
        self.next();
    }

    fn int(&mut self) -> Expr {
        let begin = self.cur;
        self.next();
        while !self.at_end() && is_digit(self.peek()) {
            self.next();
        }
        let tok = &self.src[begin..self.cur];
        let n = i64::from_str_radix(tok, 10).unwrap();
        Expr::Num(n)
    }

    fn paren(&mut self) -> Expr {
        self.eat('(');
        let expr = self.binop();
        self.eat(')');
        expr
    }

    fn operand(&mut self) -> Expr {
        if self.peek() == '(' {
            self.paren()
        } else {
            self.int()
        }
    }

    fn binop(&mut self) -> Expr {
        let first = self.operand();
        if self.at_end() || !is_op(self.peek()) {
            return first;
        }
        let mut op = self.peek();
        let mut operands = vec![first];
        while !self.at_end() && is_op(self.peek()) {
            let op1 = self.next();
            if op1 == op {
                operands.push(self.operand());
            } else {
                operands = vec![Expr::BinOp(op, operands)];
                operands.push(self.operand());
            }
            op = op1;
        }
        let binop = Expr::BinOp(op, operands);
        binop
    }
}

fn parse(s: &str) -> Expr {
    Parser::new(s).binop()
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::BinOp(op, operands) => {
            let args = operands.iter().map(eval);
            match op {
                '+' => args.sum(),
                '*' => args.product(),
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let sum: i64 = text.lines().map(parse).map(|e| eval(&e)).sum();
    println!("{}", sum);
}
