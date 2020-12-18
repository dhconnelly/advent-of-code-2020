use std::collections::HashMap;

#[derive(Debug)]
enum Expr {
    Num(i64),
    BinOp(char, Box<Expr>, Box<Expr>),
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
    prec: HashMap<char, i64>,
    max_prec: i64,
}

impl Parser {
    fn new(s: &str, prec: &HashMap<char, i64>) -> Parser {
        Parser {
            src: s.replace(' ', ""),
            text: s.replace(' ', "").chars().collect(),
            cur: 0,
            prec: prec.clone(),
            max_prec: *prec.values().max().unwrap(),
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
        let expr = self.expr();
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

    fn binop_above(&mut self, prec: i64) -> Expr {
        if prec > self.max_prec {
            let operand = self.operand();
            return operand;
        }
        let mut lhs = self.binop_above(prec + 1);
        while !self.at_end() && is_op(self.peek()) {
            let op = self.next();
            let rhs = self.binop_above(self.prec[&op] + 1);
            lhs = Expr::BinOp(op, Box::new(lhs), Box::new(rhs));
        }
        lhs
    }

    fn expr(&mut self) -> Expr {
        self.binop_above(0)
    }
}

fn parse(s: &str, prec: &HashMap<char, i64>) -> Expr {
    Parser::new(s, prec).expr()
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::BinOp(op, lhs, rhs) => match op {
            '+' => eval(lhs) + eval(rhs),
            '*' => eval(lhs) * eval(rhs),
            _ => unreachable!(),
        },
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();

    let mut prec = HashMap::new();
    prec.insert('+', 1);
    prec.insert('*', 1);
    let sum: i64 = text.lines().map(|s| eval(&parse(s, &prec))).sum();
    println!("{}", sum);

    prec.insert('+', 2);
    let sum: i64 = text.lines().map(|s| eval(&parse(s, &prec))).sum();
    println!("{}", sum);
}
