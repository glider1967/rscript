use crate::{
    expression::{ExprSpan, SpannedExpr},
    tokenize::{Span, Token, TokenType, Tokenizer},
    types::Type,
};

use anyhow::{Ok, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("at {span:?}: Unexpected token: expected {expected:?}, found {found:?}")]
    UnexpectedToken {
        expected: TokenType,
        found: Option<TokenType>,
        span: Option<Span>,
    },

    #[error("at {span:?}: Expected primary token, found {found:?}")]
    ExpectedPrimaryToken {
        found: Option<TokenType>,
        span: Option<Span>,
    },

    #[error("at {span:?}: Expected identifier, found {found:?}")]
    ExpectedIdentifier {
        found: Option<TokenType>,
        span: Option<Span>,
    },

    #[error("at {span:?} Invalid type: {found}")]
    InvalidType { found: String, span: Span },

    #[error("at {span:?} Expected type declaration, found {found:?}")]
    ExpectedType {
        found: Option<TokenType>,
        span: Option<Span>,
    },
}
#[derive(Debug)]
struct TokenInfo<T> {
    value: T,
    span: Span,
}

pub struct Parser {
    tokens: Vec<Token>,
}

macro_rules! sym {
    ($s:expr) => {
        TokenType::Symbol($s.to_string())
    };
}
macro_rules! kwd {
    ($s:expr) => {
        TokenType::Keyword($s.to_string())
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

    fn consume_int(&mut self) -> Option<TokenInfo<i64>> {
        if let Some(Token {
            ttype: TokenType::Int(val),
            span,
        }) = self.tokens.last()
        {
            let span = span.clone();
            let val = *val;
            self.tokens.pop();
            Some(TokenInfo {
                value: val,
                span: span,
            })
        } else {
            None
        }
    }

    fn consume_bool(&mut self) -> Option<TokenInfo<bool>> {
        if let Some(Token {
            ttype: TokenType::Keyword(val),
            span,
        }) = self.tokens.last()
        {
            let is_bool = val == "true" || val == "false";
            if is_bool {
                let span = span.clone();
                let val = val == "true";
                self.tokens.pop();
                Some(TokenInfo {
                    value: val,
                    span: span,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn consume_ident(&mut self) -> Option<TokenInfo<String>> {
        if let Some(Token {
            ttype: TokenType::Ident(val),
            span,
        }) = self.tokens.last()
        {
            let span = span.clone();
            let val = val.clone();
            self.tokens.pop();
            Some(TokenInfo {
                value: val,
                span: span,
            })
        } else {
            None
        }
    }

    fn expect(&mut self, expected: TokenType) -> Result<Token> {
        if let Some(t) = self.tokens.pop() {
            if t.ttype != expected {
                Err(ParseError::UnexpectedToken {
                    expected,
                    found: Some(t.ttype),
                    span: Some(t.span),
                }
                .into())
            } else {
                Ok(t)
            }
        } else {
            Err(ParseError::UnexpectedToken {
                expected,
                found: None,
                span: None,
            }
            .into())
        }
    }

    fn consume(&mut self, ttype: TokenType) -> Option<Span> {
        if let Some(t) = self.tokens.last() {
            if t.ttype == ttype {
                let token = self.tokens.pop().unwrap();
                Some(token.span)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn expect_ident(&mut self) -> Result<String> {
        if let Some(token) = self.tokens.pop() {
            match token.ttype {
                TokenType::Ident(val) => Ok(val),
                other => Err(ParseError::ExpectedIdentifier {
                    found: Some(other),
                    span: Some(token.span),
                }
                .into()),
            }
        } else {
            Err(ParseError::ExpectedIdentifier {
                found: None,
                span: None,
            }
            .into())
        }
    }

    fn expr(&mut self) -> Result<SpannedExpr> {
        self.parse_if()
    }

    fn ident_and_opt_types(&mut self) -> Result<Vec<(String, Option<Type>)>> {
        let ident = self.expect_ident()?;
        let ty = if let Some(_) = self.consume(sym!(":")) {
            Some(self.parse_ty()?)
        } else {
            None
        };
        let mut list = vec![(ident, ty)];
        loop {
            if let Some(_) = self.consume(sym!(",")) {
                let ident = self.expect_ident()?;
                let ty = if let Some(_) = self.consume(sym!(":")) {
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

    fn primary(&mut self) -> Result<SpannedExpr> {
        if let Some(start_span) = self.consume(kwd!("lambda")) {
            self.expect(sym!("("))?;
            let idents = self.ident_and_opt_types()?;
            self.expect(sym!(")"))?;
            self.expect(sym!("{"))?;
            let prog = self.prog()?;
            let end_token = self.expect(sym!("}"))?;

            let mut ret = prog;
            for (ident, ty) in idents.into_iter().rev() {
                let ret_span = ret.span.clone();
                ret = SpannedExpr::lambda(ident, ty, ret, ret_span);
            }

            Ok(SpannedExpr::new(
                ret.expr,
                ExprSpan::from_two_span(&start_span, &end_token.span),
            ))
        } else if let Some(start_span) = self.consume(sym!("(")) {
            let exp = self.expr()?;
            let end_token = self.expect(sym!(")"))?;

            Ok(SpannedExpr::new(
                exp.expr,
                ExprSpan::from_two_span(&start_span, &end_token.span),
            ))
        } else if let Some(token) = self.consume_int() {
            Ok(SpannedExpr::int(token.value, token.span))
        } else if let Some(token) = self.consume_bool() {
            Ok(SpannedExpr::boolean(token.value, token.span))
        } else if let Some(token) = self.consume_ident() {
            Ok(SpannedExpr::variable(token.value, token.span))
        } else {
            if let Some(tok) = self.tokens.pop() {
                Err(ParseError::ExpectedPrimaryToken {
                    found: Some(tok.ttype),
                    span: Some(tok.span),
                }
                .into())
            } else {
                Err(ParseError::ExpectedPrimaryToken {
                    found: None,
                    span: None,
                }
                .into())
            }
        }
    }

    fn parse_if(&mut self) -> Result<SpannedExpr> {
        if let Some(start_span) = self.consume(kwd!("if")) {
            self.expect(sym!("("))?;
            let cond = self.or()?;
            self.expect(sym!(")"))?;
            self.expect(sym!("{"))?;
            let exp1 = self.or()?;
            self.expect(sym!("}"))?;
            self.expect(kwd!("else"))?;
            self.expect(sym!("{"))?;
            let exp2 = self.or()?;
            let end_token = self.expect(sym!("}"))?;
            Ok(SpannedExpr::if_expr(
                cond,
                exp1,
                exp2,
                ExprSpan::from_two_span(&start_span, &end_token.span),
            ))
        } else {
            self.or()
        }
    }

    fn or(&mut self) -> Result<SpannedExpr> {
        let mut ret = self.and()?;
        loop {
            if let Some(_) = self.consume(sym!("||")) {
                let exp = self.and()?;
                ret = SpannedExpr::binary_op("||".into(), ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn and(&mut self) -> Result<SpannedExpr> {
        let mut ret = self.equ()?;
        loop {
            if let Some(_) = self.consume(sym!("&&")) {
                let exp = self.equ()?;
                ret = SpannedExpr::binary_op("&&".into(), ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn equ(&mut self) -> Result<SpannedExpr> {
        let mut ret = self.rel()?;
        let mut now;
        let mut prev;

        if let Some(_) = self.consume(sym!("==")) {
            now = self.rel()?.clone();
            ret = SpannedExpr::binary_op("==".to_owned(), ret, now.clone());
        } else if let Some(_) = self.consume(sym!("!=".to_owned())) {
            now = self.rel()?.clone();
            ret = SpannedExpr::binary_op("!=".into(), ret, now.clone());
        } else {
            return Ok(ret);
        }

        loop {
            if let Some(_) = self.consume(sym!("==")) {
                prev = now;
                now = self.rel()?.clone();
                ret = SpannedExpr::binary_op(
                    "==".into(),
                    ret,
                    SpannedExpr::binary_op("==".into(), prev.clone(), now.clone()),
                );
            } else if let Some(_) = self.consume(sym!("!=")) {
                prev = now;
                now = self.rel()?.clone();
                ret = SpannedExpr::binary_op(
                    "!=".into(),
                    ret,
                    SpannedExpr::binary_op("!=".into(), prev.clone(), now.clone()),
                );
            } else {
                return Ok(ret);
            }
        }
    }

    fn rel(&mut self) -> Result<SpannedExpr> {
        let mut ret = self.add()?;
        let mut now: SpannedExpr;
        let mut prev: SpannedExpr;

        if let Some(_) = self.consume(sym!("<")) {
            now = self.add()?;
            ret = SpannedExpr::binary_op("<".into(), ret, now.clone());
        } else if let Some(_) = self.consume(sym!(">")) {
            now = self.add()?;
            ret = SpannedExpr::binary_op(">".into(), ret, now.clone());
        } else if let Some(_) = self.consume(sym!("<=")) {
            now = self.add()?;
            ret = SpannedExpr::binary_op("<=".into(), ret, now.clone());
        } else if let Some(_) = self.consume(sym!(">=")) {
            now = self.add()?;
            ret = SpannedExpr::binary_op(">=".into(), ret, now.clone());
        } else {
            return Ok(ret);
        }

        loop {
            if let Some(_) = self.consume(sym!("<")) {
                prev = now;
                now = self.add()?;
                let inner_op = SpannedExpr::binary_op("<".into(), prev.clone(), now.clone());
                ret = SpannedExpr::binary_op("<".into(), ret.clone(), inner_op);
            } else if let Some(_) = self.consume(sym!(">")) {
                prev = now;
                now = self.add()?;
                let inner_op = SpannedExpr::binary_op(">".into(), prev.clone(), now.clone());
                ret = SpannedExpr::binary_op(">".into(), ret.clone(), inner_op);
            } else if let Some(_) = self.consume(sym!("<=")) {
                prev = now;
                now = self.add()?;
                let inner_op = SpannedExpr::binary_op("<=".into(), prev.clone(), now.clone());
                ret = SpannedExpr::binary_op("<=".into(), ret.clone(), inner_op);
            } else if let Some(_) = self.consume(sym!(">=")) {
                prev = now;
                now = self.add()?;
                let inner_op = SpannedExpr::binary_op(">=".into(), prev.clone(), now.clone());
                ret = SpannedExpr::binary_op(">=".into(), ret.clone(), inner_op);
            } else {
                return Ok(ret);
            }
        }
    }

    fn add(&mut self) -> Result<SpannedExpr> {
        let mut ret = self.mul()?;
        loop {
            if let Some(_) = self.consume(sym!("+")) {
                let exp = self.mul()?;
                ret = SpannedExpr::binary_op("+".to_owned(), ret, exp);
            } else if let Some(_) = self.consume(sym!("-")) {
                let exp = self.mul()?;
                ret = SpannedExpr::binary_op("-".to_owned(), ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn mul(&mut self) -> Result<SpannedExpr> {
        let mut ret = self.unary()?;
        loop {
            if let Some(_) = self.consume(sym!("*")) {
                let exp = self.unary()?;
                ret = SpannedExpr::binary_op("*".to_owned(), ret, exp);
            } else if let Some(_) = self.consume(sym!("/")) {
                let exp = self.unary()?;
                ret = SpannedExpr::binary_op("/".to_owned(), ret, exp);
            } else {
                return Ok(ret);
            }
        }
    }

    fn unary(&mut self) -> Result<SpannedExpr> {
        if let Some(sp) = self.consume(sym!("-")) {
            Ok(SpannedExpr::unary_op("-".into(), self.app()?, sp))
        } else if let Some(sp) = self.consume(sym!("!")) {
            Ok(SpannedExpr::unary_op("!".into(), self.app()?, sp))
        } else {
            Ok(self.app()?)
        }
    }

    fn app(&mut self) -> Result<SpannedExpr> {
        let mut ret = self.primary()?;
        if let Some(_) = self.consume(sym!("(")) {
            let var = self.expr()?;
            let span = ExprSpan::from_two_exprspan(&ret.span, &var.span);
            ret = SpannedExpr::app(ret, var, span);
            loop {
                if let Some(_) = self.consume(sym!(",")) {
                    let var = self.expr()?;
                    let span = ExprSpan::from_two_exprspan(&ret.span, &var.span);
                    ret = SpannedExpr::app(ret, var, span);
                } else {
                    break;
                }
            }
            self.expect(sym!(")"))?;
        }
        Ok(ret)
    }

    pub fn prog(&mut self) -> Result<SpannedExpr> {
        let mut prog = vec![];
        loop {
            if let Some(start_span) = self.consume(kwd!("let")) {
                let ident = self.expect_ident()?;
                let ty = if let Some(_) = self.consume(sym!(":")) {
                    Some(self.parse_ty()?)
                } else {
                    None
                };
                self.expect(sym!("="))?;
                let expr = self.expr()?;
                let end_token = self.expect(sym!(";"))?;
                prog.push(SpannedExpr::assign(
                    ident,
                    ty,
                    expr,
                    ExprSpan::from_two_span(&start_span, &end_token.span),
                ));
            } else if let Some(start_span) = self.consume(kwd!("mut")) {
                let ident = self.expect_ident()?;
                self.expect(sym!("="))?;
                let expr = self.expr()?;
                let end_token = self.expect(sym!(";"))?;
                prog.push(SpannedExpr::reassign(
                    ident,
                    expr,
                    ExprSpan::from_two_span(&start_span, &end_token.span),
                ));
            } else {
                break;
            }
        }

        let ret = self.expr()?;
        let ret_span = if prog.is_empty() {
            ret.span.clone()
        } else {
            ExprSpan::from_two_exprspan(&prog[0].span, &ret.span)
        };
        Ok(SpannedExpr::program(prog, ret, ret_span))
    }

    // =====================================================================

    fn parse_ty(&mut self) -> Result<Type> {
        self.fntype()
    }

    fn fntype(&mut self) -> Result<Type> {
        let mut list = vec![self.primary_type()?];
        loop {
            if let Some(_) = self.consume(sym!("->")) {
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
        if let Some(_) = self.consume(sym!("(")) {
            let ty = self.parse_ty()?;
            self.expect(sym!(")"))?;
            Ok(ty)
        } else {
            match self.tokens.pop() {
                Some(Token {
                    ttype: TokenType::Type(val),
                    span,
                }) => match val.as_str() {
                    "int" | "bool" => Ok(Type::constant(&val)),
                    _ => Err(ParseError::InvalidType { found: val, span }.into()),
                },
                Some(Token { ttype, span }) => Err(ParseError::ExpectedType {
                    found: Some(ttype),
                    span: Some(span),
                }
                .into()),
                None => Err(ParseError::ExpectedType {
                    found: None,
                    span: None,
                }
                .into()),
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
        let expr: Expr = Parser::new("233425").expr().unwrap().expr;
        assert_eq!(expr, Expr::Int(233425),);
    }
}
