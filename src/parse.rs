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
        self.parse_expr()
    }

    fn parse_primary(&mut self) -> Expr {
        if self.consume(Token::LParen) {
            let exp = self.parse_expr();
            self.expect(Token::RParen);
            exp
        } else {
            let num = self.expect_int();
            Expr::int(num)
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let mut ret = self.parse_mul();
        loop {
            if self.consume(Token::Plus) {
                let exp = self.parse_mul();
                ret = Expr::plus(ret, exp);
            } else if self.consume(Token::Minus) {
                let exp = self.parse_mul();
                ret = Expr::minus(ret, exp);
            } else {
                return ret;
            }
        }
    }

    fn parse_mul(&mut self) -> Expr {
        let mut ret = self.parse_primary();
        loop {
            if self.consume(Token::Star) {
                let exp = self.parse_primary();
                ret = Expr::mult(ret, exp);
            } else if self.consume(Token::Div) {
                let exp = self.parse_primary();
                ret = Expr::div(ret, exp);
            } else {
                return ret;
            }
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
