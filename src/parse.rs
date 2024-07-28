use crate::ast::Expr;

pub struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
            pos: 0,
        }
    }

    fn consume(&mut self, ops: Vec<char>) -> Option<char> {
        if let Some(ch) = self.input.chars().nth(self.pos) {
            if !ops.contains(&ch) {
                None
            } else {
                self.pos += 1;
                Some(ch)
            }
        } else {
            None
        }
    }

    fn expect(&mut self, op: char) {
        if let Some(ch) = self.input.chars().nth(self.pos) {
            if ch == op {
                self.pos += 1;
            } else {
                panic!("unexpected char!")
            }
        } else {
            panic!("exceeded strlen!")
        }
    }

    fn expect_numchar(&mut self) -> Option<char> {
        if let Some(ch) = self.input.chars().nth(self.pos) {
            if ch.is_digit(10) {
                self.pos += 1;
                return Some(ch);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_expr()
    }

    fn parse_primary(&mut self) -> Expr {
        if let Some(_) = self.consume(vec!['(']) {
            let exp = self.parse_expr();
            self.expect(')');
            exp
        } else {
            self.parse_int()
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let mut ret = self.parse_primary();
        while let Some(op) = self.consume(vec!['+', '-']) {
            let exp = self.parse_primary();
            match op {
                '+' => {
                    ret = Expr::plus(ret, exp);
                }
                '-' => {
                    ret = Expr::minus(ret, exp);
                }
                _ => unreachable!(),
            }
        }
        ret
    }

    fn parse_int(&mut self) -> Expr {
        let mut numstr = String::new();
        while let Some(numch) = self.expect_numchar() {
            numstr.push(numch);
        }

        let num: i64 = numstr.parse().unwrap();
        Expr::int(num)
    }
}

#[cfg(test)]
mod parse {
    use crate::ast::Expr;

    use super::Parser;

    #[test]
    fn parse_num() {
        let expr: Expr = Parser::new("233425").parse();
        assert_eq!(expr, Expr::Int(233425));
    }
}
