use crate::{
    expression::Expr,
    statement::Statement,
    tokenize::{Token, Tokenizer},
};

use anyhow::{bail, Ok, Result};

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

    fn expect(&mut self, token: Token) -> Result<()> {
        if let Some(t) = self.tokens.pop() {
            if t != token {
                bail!("unexpected token")
            } else {
                Ok(())
            }
        } else {
            bail!("unexpected EOF")
        }
    }

    fn consume_int(&mut self) -> Option<i64> {
        if let Some(Token::Int(val)) = self.tokens.last() {
            let r = Some(*val);
            let _ = self.tokens.pop();
            r
        } else {
            None
        }
    }

    fn consume_bool(&mut self) -> Option<bool> {
        if let Some(Token::Keyword(val)) = self.tokens.last() {
            if *val == "true" {
                let _ = self.tokens.pop();
                Some(true)
            } else if *val == "false" {
                let _ = self.tokens.pop();
                Some(false)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn consume_ident(&mut self) -> Option<String> {
        if let Some(Token::Ident(val)) = self.tokens.last() {
            let r = Some(val.clone());
            let _ = self.tokens.pop();
            r
        } else {
            None
        }
    }

    fn expect_ident(&mut self) -> Result<String> {
        if let Some(Token::Ident(val)) = self.tokens.pop() {
            Ok(val)
        } else {
            bail!("unexpected non-identifier")
        }
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_if()
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.consume(Token::Symbol("(".to_owned())) {
            let exp = self.parse_expr();
            self.expect(Token::Symbol(")".to_owned()))?;
            exp
        } else {
            if let Some(num) = self.consume_int() {
                Ok(Expr::int(num))
            } else {
                if let Some(b) = self.consume_bool() {
                    Ok(Expr::boolean(b))
                } else {
                    if let Some(name) = self.consume_ident() {
                        Ok(Expr::ident(name))
                    } else {
                        bail!("unexpected token")
                    }
                }
            }
        }
    }

    fn parse_if(&mut self) -> Result<Expr> {
        if self.consume(Token::Keyword("if".to_string())) {
            self.expect(Token::Symbol("(".to_string()))?;
            let cond = self.or()?;
            self.expect(Token::Symbol(")".to_string()))?;
            self.expect(Token::Symbol("{".to_string()))?;
            let exp1 = self.or()?;
            self.expect(Token::Symbol("}".to_string()))?;
            self.expect(Token::Keyword("else".to_string()))?;
            self.expect(Token::Symbol("{".to_string()))?;
            let exp2 = self.or()?;
            self.expect(Token::Symbol("}".to_string()))?;
            Ok(Expr::if_expr(cond, exp1, exp2))
        } else {
            self.or()
        }
    }

    fn or(&mut self) -> Result<Expr> {
        let mut ret = self.and()?;
        loop {
            if self.consume(Token::Symbol("||".to_string())) {
                let exp = self.and()?;
                ret = Expr::bin_or(ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn and(&mut self) -> Result<Expr> {
        let mut ret = self.equ()?;
        loop {
            if self.consume(Token::Symbol("&&".to_string())) {
                let exp = self.equ()?;
                ret = Expr::bin_and(ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn equ(&mut self) -> Result<Expr> {
        let mut ret = self.rel()?;
        let mut now;
        let mut prev;

        if self.consume(Token::Symbol("==".to_owned())) {
            now = self.rel()?.clone();
            ret = Expr::bin_eq(ret, now.clone());
        } else if self.consume(Token::Symbol("!=".to_owned())) {
            now = self.rel()?.clone();
            ret = Expr::bin_neq(ret, now.clone());
        } else {
            return Ok(ret);
        }

        loop {
            if self.consume(Token::Symbol("==".to_owned())) {
                prev = now;
                now = self.rel()?.clone();
                ret = Expr::bin_and(ret, Expr::bin_eq(prev.clone(), now.clone()));
            } else if self.consume(Token::Symbol("!=".to_owned())) {
                prev = now;
                now = self.rel()?.clone();
                ret = Expr::bin_and(ret, Expr::bin_neq(prev.clone(), now.clone()));
            } else {
                return Ok(ret);
            }
        }
    }

    fn rel(&mut self) -> Result<Expr> {
        let mut ret = self.add()?;
        let mut now;
        let mut prev;

        if self.consume(Token::Symbol("<".to_owned())) {
            now = self.add()?.clone();
            ret = Expr::bin_lt(ret, now.clone());
        } else if self.consume(Token::Symbol(">".to_owned())) {
            now = self.add()?.clone();
            ret = Expr::bin_gt(ret, now.clone());
        } else if self.consume(Token::Symbol("<=".to_owned())) {
            now = self.add()?.clone();
            ret = Expr::bin_le(ret, now.clone());
        } else if self.consume(Token::Symbol(">=".to_owned())) {
            now = self.add()?.clone();
            ret = Expr::bin_ge(ret, now.clone());
        } else {
            return Ok(ret);
        }

        loop {
            if self.consume(Token::Symbol("<".to_owned())) {
                prev = now;
                now = self.add()?.clone();
                ret = Expr::bin_and(ret, Expr::bin_lt(prev.clone(), now.clone()));
            } else if self.consume(Token::Symbol(">".to_owned())) {
                prev = now;
                now = self.add()?.clone();
                ret = Expr::bin_and(ret, Expr::bin_gt(prev.clone(), now.clone()));
            } else if self.consume(Token::Symbol("<=".to_owned())) {
                prev = now;
                now = self.add()?.clone();
                ret = Expr::bin_and(ret, Expr::bin_le(prev.clone(), now.clone()));
            } else if self.consume(Token::Symbol(">=".to_owned())) {
                prev = now;
                now = self.add()?.clone();
                ret = Expr::bin_and(ret, Expr::bin_ge(prev.clone(), now.clone()));
            } else {
                return Ok(ret);
            }
        }
    }

    fn add(&mut self) -> Result<Expr> {
        let mut ret = self.mul()?;
        loop {
            if self.consume(Token::Symbol("+".to_string())) {
                let exp = self.mul()?;
                ret = Expr::bin_plus(ret, exp);
            } else if self.consume(Token::Symbol("-".to_string())) {
                let exp = self.mul()?;
                ret = Expr::bin_minus(ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn mul(&mut self) -> Result<Expr> {
        let mut ret = self.unary()?;
        loop {
            if self.consume(Token::Symbol("*".to_owned())) {
                let exp = self.unary()?;
                ret = Expr::bin_mult(ret, exp);
            } else if self.consume(Token::Symbol("/".to_owned())) {
                let exp = self.unary()?;
                ret = Expr::bin_div(ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.consume(Token::Symbol("-".to_owned())) {
            Ok(Expr::unary_minus(self.primary()?))
        } else if self.consume(Token::Symbol("!".to_owned())) {
            Ok(Expr::unary_not(self.primary()?))
        } else {
            Ok(self.primary()?)
        }
    }

    pub fn parse_stmt(&mut self) -> Result<Statement> {
        if self.consume(Token::Keyword("let".to_owned())) {
            let ident = self.expect_ident()?;
            self.expect(Token::Symbol("=".to_owned()))?;
            let expr = self.parse_expr()?;
            self.expect(Token::Symbol(";".to_string()))?;
            Ok(Statement::AssignAndConseq(
                ident,
                expr,
                Box::new(self.parse_stmt()?),
            ))
        } else {
            Ok(Statement::Expression(self.parse_expr()?))
        }
    }
}

#[cfg(test)]
mod parse {
    use crate::expression::Expr;

    use super::Parser;

    #[test]
    fn parse_num() {
        let expr: Expr = Parser::new("233425").parse_expr().unwrap();
        assert_eq!(expr, Expr::Int(233425));
    }
}
