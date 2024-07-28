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

    fn consume(&mut self, op: char) -> bool {
        if let Some(ch) = self.input.chars().nth(self.pos) {
            if ch == op {
                return false;
            } else {
                self.pos += 1;
                return true;
            }
        } else {
            panic!("exceeded strlen!")
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

    fn parse_expr(&mut self) -> Expr {
        self.parse_int()
    }

    fn parse_int(&mut self) -> Expr {
        let mut numstr = String::new();
        while let Some(numch) = self.expect_numchar() {
            numstr.push(numch);
        }

        let num: u64 = numstr.parse().unwrap();
        Expr::Int(num)
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
