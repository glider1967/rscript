use crate::{
    ast::Expr,
    tokenize::{Token, Tokenizer},
};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            tokens: Tokenizer::new(input)
                .tokenize()
                .iter()
                .rev()
                .cloned()
                .collect(),
        }
    }

    fn consume(&mut self, token: Token) -> bool {
        if let Some(t) = self.tokens.last() {
            if *t == token {
                let _ = self.tokens.pop();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn expect(&mut self, token: Token) {
        if let Some(t) = self.tokens.pop() {
            if t != token {
                panic!();
            }
        } else {
            panic!();
        }
    }

    fn expect_int(&mut self) -> i64 {
        if let Some(Token::Int(val)) = self.tokens.pop() {
            return val;
        } else {
            panic!();
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.equ()
    }

    fn primary(&mut self) -> Expr {
        if self.consume(Token::LParen) {
            let exp = self.add();
            self.expect(Token::RParen);
            exp
        } else {
            let num = self.expect_int();
            Expr::int(num)
        }
    }

    fn equ(&mut self) -> Expr {
        let mut ret = self.rel();
        let mut now;
        let mut prev;

        if self.consume(Token::Eq) {
            now = self.mul().clone();
            ret = Expr::bin_eq(ret, now.clone());
        } else if self.consume(Token::NotEq) {
            now = self.mul().clone();
            ret = Expr::bin_neq(ret, now.clone());
        } else {
            return ret;
        }

        loop {
            if self.consume(Token::Eq) {
                prev = now;
                now = self.mul().clone();
                ret = Expr::bin_and(ret, Expr::bin_eq(prev.clone(), now.clone()));
            } else if self.consume(Token::NotEq) {
                prev = now;
                now = self.mul().clone();
                ret = Expr::bin_and(ret, Expr::bin_neq(prev.clone(), now.clone()));
            } else {
                return ret;
            }
        }
    }

    fn rel(&mut self) -> Expr {
        let mut ret = self.add();
        loop {
            if self.consume(Token::LParenA) {
                let exp = self.add();
                ret = Expr::bin_lt(ret, exp);
            } else if self.consume(Token::RParenA) {
                let exp = self.add();
                ret = Expr::bin_gt(ret, exp);
            } else if self.consume(Token::LessEq) {
                let exp = self.add();
                ret = Expr::bin_le(ret, exp);
            } else if self.consume(Token::GreaterEq) {
                let exp = self.add();
                ret = Expr::bin_ge(ret, exp);
            } else {
                return ret;
            }
        }
    }

    fn add(&mut self) -> Expr {
        let mut ret = self.mul();
        loop {
            if self.consume(Token::Plus) {
                let exp = self.mul();
                ret = Expr::bin_plus(ret, exp);
            } else if self.consume(Token::Minus) {
                let exp = self.mul();
                ret = Expr::bin_minus(ret, exp);
            } else {
                return ret;
            }
        }
    }

    fn mul(&mut self) -> Expr {
        let mut ret = self.unary();
        loop {
            if self.consume(Token::Star) {
                let exp = self.unary();
                ret = Expr::bin_mult(ret, exp);
            } else if self.consume(Token::Div) {
                let exp = self.unary();
                ret = Expr::bin_div(ret, exp);
            } else {
                return ret;
            }
        }
    }

    fn unary(&mut self) -> Expr {
        let ret = self.primary();
        if self.consume(Token::Minus) {
            Expr::UnaryMinus(Box::new(ret))
        } else {
            ret
        }
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
