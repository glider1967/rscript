use crate::{
    expression::Expr,
    tokenize::{Token, Tokenizer},
    types::Type,
};

use anyhow::{bail, Ok, Result};

pub struct Parser {
    tokens: Vec<Token>,
}

macro_rules! sym {
    ($s:expr) => {
        Token::Symbol($s.to_string())
    };
}
macro_rules! kwd {
    ($s:expr) => {
        Token::Keyword($s.to_string())
    };
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
                bail!("unexpected token: {:?}", t)
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

    fn expr(&mut self) -> Result<Expr> {
        self.parse_if()
    }

    fn ident_and_opt_types(&mut self) -> Result<Vec<(String, Option<Type>)>> {
        let ident = self.expect_ident()?;
        let ty = if self.consume(sym!(":")) {
            Some(self.parse_ty()?)
        } else {
            None
        };
        let mut list = vec![(ident, ty)];
        loop {
            if self.consume(sym!(",")) {
                let ident = self.expect_ident()?;
                let ty = if self.consume(sym!(":")) {
                    Some(self.parse_ty()?)
                } else {
                    None
                };
                list.push((ident, ty));
            } else {
                break;
            }
        }
        Ok(list)
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.consume(kwd!("lambda")) {
            self.expect(sym!("("))?;
            let idents = self.ident_and_opt_types()?;
            self.expect(sym!(")"))?;
            self.expect(sym!("{"))?;
            let prog = self.prog()?;
            self.expect(sym!("}"))?;
            let mut ret = prog;
            for (ident, ty) in idents.into_iter().rev() {
                ret = Expr::lambda(ident, ty, ret);
            }
            Ok(ret)
        } else if self.consume(sym!("(")) {
            let exp = self.expr();
            self.expect(sym!(")"))?;
            exp
        } else if let Some(num) = self.consume_int() {
            Ok(Expr::int(num))
        } else if let Some(b) = self.consume_bool() {
            Ok(Expr::boolean(b))
        } else if let Some(name) = self.consume_ident() {
            Ok(Expr::variable(name))
        } else {
            bail!("unexpected token: {:?}", self.tokens.last())
        }
    }

    fn parse_if(&mut self) -> Result<Expr> {
        if self.consume(kwd!("if")) {
            self.expect(sym!("("))?;
            let cond = self.or()?;
            self.expect(sym!(")"))?;
            self.expect(sym!("{"))?;
            let exp1 = self.or()?;
            self.expect(sym!("}"))?;
            self.expect(kwd!("else"))?;
            self.expect(sym!("{"))?;
            let exp2 = self.or()?;
            self.expect(sym!("}"))?;
            Ok(Expr::if_expr(cond, exp1, exp2))
        } else {
            self.or()
        }
    }

    fn or(&mut self) -> Result<Expr> {
        let mut ret = self.and()?;
        loop {
            if self.consume(sym!("||")) {
                let exp = self.and()?;
                ret = Expr::binop("||".into(), ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn and(&mut self) -> Result<Expr> {
        let mut ret = self.equ()?;
        loop {
            if self.consume(sym!("&&")) {
                let exp = self.equ()?;
                ret = Expr::binop("&&".into(), ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn equ(&mut self) -> Result<Expr> {
        let mut ret = self.rel()?;
        let mut now;
        let mut prev;

        if self.consume(sym!("==")) {
            now = self.rel()?.clone();
            ret = Expr::binop("==".to_owned(), ret, now.clone());
        } else if self.consume(sym!("!=".to_owned())) {
            now = self.rel()?.clone();
            ret = Expr::binop("!=".into(), ret, now.clone());
        } else {
            return Ok(ret);
        }

        loop {
            if self.consume(sym!("==")) {
                prev = now;
                now = self.rel()?.clone();
                ret = Expr::binop(
                    "==".into(),
                    ret,
                    Expr::binop("==".into(), prev.clone(), now.clone()),
                );
            } else if self.consume(sym!("!=")) {
                prev = now;
                now = self.rel()?.clone();
                ret = Expr::binop(
                    "!=".into(),
                    ret,
                    Expr::binop("!=".into(), prev.clone(), now.clone()),
                );
            } else {
                return Ok(ret);
            }
        }
    }

    fn rel(&mut self) -> Result<Expr> {
        let mut ret = self.add()?;
        let mut now;
        let mut prev;

        if self.consume(sym!("<")) {
            now = self.add()?.clone();
            ret = Expr::binop("<".into(), ret, now.clone());
        } else if self.consume(sym!(">")) {
            now = self.add()?.clone();
            ret = Expr::binop(">".into(), ret, now.clone());
        } else if self.consume(sym!("<=")) {
            now = self.add()?.clone();
            ret = Expr::binop("<=".into(), ret, now.clone());
        } else if self.consume(sym!(">=")) {
            now = self.add()?.clone();
            ret = Expr::binop(">=".into(), ret, now.clone());
        } else {
            return Ok(ret);
        }

        loop {
            if self.consume(sym!("<")) {
                prev = now;
                now = self.add()?.clone();
                ret = Expr::binop(
                    "<".into(),
                    ret,
                    Expr::binop("<".into(), prev.clone(), now.clone()),
                );
            } else if self.consume(sym!(">")) {
                prev = now;
                now = self.add()?.clone();
                ret = Expr::binop(
                    ">".into(),
                    ret,
                    Expr::binop(">".into(), prev.clone(), now.clone()),
                );
            } else if self.consume(sym!("<=")) {
                prev = now;
                now = self.add()?.clone();
                ret = Expr::binop(
                    "<=".into(),
                    ret,
                    Expr::binop("<=".into(), prev.clone(), now.clone()),
                );
            } else if self.consume(sym!(">=")) {
                prev = now;
                now = self.add()?.clone();
                ret = Expr::binop(
                    ">=".into(),
                    ret,
                    Expr::binop(">=".into(), prev.clone(), now.clone()),
                );
            } else {
                return Ok(ret);
            }
        }
    }

    fn add(&mut self) -> Result<Expr> {
        let mut ret = self.mul()?;
        loop {
            if self.consume(sym!("+")) {
                let exp = self.mul()?;
                ret = Expr::binop("+".to_owned(), ret, exp);
            } else if self.consume(sym!("-")) {
                let exp = self.mul()?;
                ret = Expr::binop("-".to_owned(), ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn mul(&mut self) -> Result<Expr> {
        let mut ret = self.unary()?;
        loop {
            if self.consume(sym!("*")) {
                let exp = self.unary()?;
                ret = Expr::binop("*".to_owned(), ret, exp);
            } else if self.consume(sym!("/")) {
                let exp = self.unary()?;
                ret = Expr::binop("/".to_owned(), ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.consume(sym!("-")) {
            Ok(Expr::unaryop("-".into(), self.app()?))
        } else if self.consume(sym!("!")) {
            Ok(Expr::unaryop("!".into(), self.app()?))
        } else {
            Ok(self.app()?)
        }
    }

    fn app(&mut self) -> Result<Expr> {
        let mut ret = self.primary()?;
        if self.consume(sym!("(")) {
            let var = self.expr()?;
            ret = Expr::app(ret, var);
            loop {
                if self.consume(sym!(",")) {
                    let var = self.expr()?;
                    ret = Expr::app(ret, var);
                } else {
                    break;
                }
            }
            self.expect(sym!(")"))?;
        }
        Ok(ret)
    }

    pub fn prog(&mut self) -> Result<Expr> {
        let mut prog = vec![];
        while self.consume(kwd!("let")) {
            let ident = self.expect_ident()?;
            let ty = if self.consume(sym!(":")) {
                Some(self.parse_ty()?)
            } else {
                None
            };
            self.expect(sym!("="))?;
            let expr = self.expr()?;
            self.expect(sym!(";"))?;
            prog.push(Expr::assign(ident, ty, expr));
        }

        let ret = self.expr()?;
        Ok(Expr::program(prog, ret))
    }

    // =====================================================================

    fn parse_ty(&mut self) -> Result<Type> {
        self.fntype()
    }

    fn fntype(&mut self) -> Result<Type> {
        let mut list = vec![self.primary_type()?];
        loop {
            if self.consume(sym!("->")) {
                let ty = self.primary_type()?;
                list.push(ty);
            } else {
                break;
            }
        }
        Ok(list
            .iter()
            .rev()
            .cloned()
            .reduce(|acc, x| Type::func(x, acc))
            .unwrap())
    }

    fn primary_type(&mut self) -> Result<Type> {
        if self.consume(sym!("(")) {
            let ty = self.parse_ty()?;
            self.expect(sym!(")"))?;
            Ok(ty)
        } else {
            if let Some(Token::Type(val)) = self.tokens.pop() {
                if &val == "int" {
                    Ok(Type::constant(&val))
                } else if &val == "bool" {
                    Ok(Type::constant(&val))
                } else {
                    bail!("unexpected type: {val}")
                }
            } else {
                bail!("unexpected non-type")
            }
        }
    }
}

#[cfg(test)]
mod parse {
    use crate::expression::Expr;

    use super::Parser;

    #[test]
    fn parse_num() {
        let expr: Expr = Parser::new("233425").expr().unwrap();
        assert_eq!(expr, Expr::Int(233425),);
    }
}
