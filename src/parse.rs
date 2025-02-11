use crate::{
    expression::{ConstructorDef, Pattern, SpannedExpr},
    span::Span,
    tokenize::{Token, TokenType, Tokenizer},
    types::Type,
};

use anyhow::{Ok, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("at {span}: Unexpected token: expected {expected:?}, found {found:?}")]
    UnexpectedToken {
        expected: TokenType,
        found: TokenType,
        span: Span,
    },

    #[error("at {span}: Expected primary token, found {found:?}")]
    ExpectedPrimaryToken { found: TokenType, span: Span },

    #[error("at {span}: Expected identifier, found {found:?}")]
    ExpectedIdentifier { found: TokenType, span: Span },

    #[error("at {span} Invalid type: {found}")]
    InvalidType { found: String, span: Span },

    #[error("at {span} Expected type declaration, found {found:?}")]
    ExpectedType { found: TokenType, span: Span },

    #[error("Expected some token, but found EOF")]
    UnexpectedEOF,
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
            let span = *span;
            let val = *val;
            self.tokens.pop();
            Some(TokenInfo { value: val, span })
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
                let span = *span;
                let val = val == "true";
                self.tokens.pop();
                Some(TokenInfo { value: val, span })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn consume_string(&mut self) -> Option<TokenInfo<String>> {
        if let Some(Token {
            ttype: TokenType::Str(val),
            span,
        }) = self.tokens.last()
        {
            let span = *span;
            let val = val.clone();
            self.tokens.pop();
            Some(TokenInfo { value: val, span })
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
            let span = *span;
            let val = val.clone();
            self.tokens.pop();
            Some(TokenInfo { value: val, span })
        } else {
            None
        }
    }

    fn expect(&mut self, expected: TokenType) -> Result<Token> {
        if let Some(t) = self.tokens.pop() {
            if t.ttype != expected {
                Err(ParseError::UnexpectedToken {
                    expected,
                    found: t.ttype,
                    span: t.span,
                }
                .into())
            } else {
                Ok(t)
            }
        } else {
            Err(ParseError::UnexpectedEOF.into())
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
                    found: other,
                    span: token.span,
                }
                .into()),
            }
        } else {
            Err(ParseError::UnexpectedEOF.into())
        }
    }

    fn expr(&mut self) -> Result<SpannedExpr> {
        self.or()
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
                let ret_span = ret.span;
                ret = SpannedExpr::lambda(ident, ty, ret, ret_span);
            }

            Ok(SpannedExpr::new(
                ret.expr,
                Span::compose(&start_span, &end_token.span),
            ))
        } else if let Some(start_span) = self.consume(kwd!("match")) {
            self.expect(sym!("("))?;
            let expr = self.expr()?;
            self.expect(sym!(")"))?;
            self.expect(sym!("{"))?;
            let arms = self.arms()?;
            let end_token = self.expect(sym!("}"))?;

            Ok(SpannedExpr::match_expr(
                expr,
                arms,
                Span::compose(&start_span, &end_token.span),
            ))
        } else if let Some(start_span) = self.consume(kwd!("if")) {
            self.if_expr(&start_span)
        } else if let Some(start_span) = self.consume(sym!("(")) {
            let exp = self.expr()?;
            let end_token = self.expect(sym!(")"))?;

            Ok(SpannedExpr::new(
                exp.expr,
                Span::compose(&start_span, &end_token.span),
            ))
        } else if let Some(token) = self.consume_int() {
            Ok(SpannedExpr::int(token.value, token.span))
        } else if let Some(token) = self.consume_bool() {
            Ok(SpannedExpr::boolean(token.value, token.span))
        } else if let Some(token) = self.consume_string() {
            Ok(SpannedExpr::string(token.value, token.span))
        } else if let Some(token) = self.consume_ident() {
            Ok(SpannedExpr::variable(token.value, token.span))
        } else {
            if let Some(tok) = self.tokens.pop() {
                Err(ParseError::ExpectedPrimaryToken {
                    found: tok.ttype,
                    span: tok.span,
                }
                .into())
            } else {
                Err(ParseError::UnexpectedEOF.into())
            }
        }
    }

    fn if_expr(&mut self, start_span: &Span) -> Result<SpannedExpr> {
        self.expect(sym!("("))?;
        let cond = self.expr()?;
        self.expect(sym!(")"))?;
        self.expect(sym!("{"))?;
        let then_expr = self.expr()?;
        self.expect(sym!("}"))?;
        self.expect(kwd!("else"))?;

        if let Some(if_span) = self.consume(kwd!("if")) {
            let else_if_expr = self.if_expr(&if_span)?;
            let end_span = else_if_expr.span;
            Ok(SpannedExpr::if_expr(
                cond,
                then_expr,
                else_if_expr,
                Span::compose(&start_span, &end_span),
            ))
        } else {
            self.expect(sym!("{"))?;
            let else_expr = self.expr()?;
            let end_token = self.expect(sym!("}"))?;
            Ok(SpannedExpr::if_expr(
                cond,
                then_expr,
                else_expr,
                Span::compose(&start_span, &end_token.span),
            ))
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
        let mut expr = self.concat()?;

        let mut prev_value = match self.next_relation_op() {
            Some(op) => {
                let value = self.concat()?;
                expr = SpannedExpr::binary_op(op.into(), expr, value.clone());
                value
            }
            None => return Ok(expr),
        };

        while let Some(next_op) = self.next_relation_op() {
            let next_value = self.concat()?;

            let next_comparison =
                SpannedExpr::binary_op(next_op.into(), prev_value.clone(), next_value.clone());

            // Combine with AND operator
            expr = SpannedExpr::binary_op("&&".into(), expr, next_comparison);
            prev_value = next_value;
        }

        Ok(expr)
    }

    fn next_relation_op(&mut self) -> Option<&'static str> {
        if let Some(_) = self.consume(sym!("<")) {
            Some("<")
        } else if let Some(_) = self.consume(sym!(">")) {
            Some(">")
        } else if let Some(_) = self.consume(sym!("<=")) {
            Some("<=")
        } else if let Some(_) = self.consume(sym!(">=")) {
            Some(">=")
        } else {
            None
        }
    }

    fn concat(&mut self) -> Result<SpannedExpr> {
        let mut ret = self.add()?;
        loop {
            if let Some(_) = self.consume(sym!("++")) {
                let exp = self.add()?;
                ret = SpannedExpr::binary_op("++".into(), ret, exp);
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
            } else if let Some(_) = self.consume(sym!("%")) {
                let exp = self.unary()?;
                ret = SpannedExpr::binary_op("%".to_owned(), ret, exp);
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
        } else if let Some(sp) = self.consume(sym!("~")) {
            Ok(SpannedExpr::unary_op("~".into(), self.app()?, sp))
        } else {
            Ok(self.app()?)
        }
    }

    fn app(&mut self) -> Result<SpannedExpr> {
        let mut ret = self.primary()?;
        if let Some(_) = self.consume(sym!("(")) {
            let var = self.expr()?;
            let span = Span::compose(&ret.span, &var.span);
            ret = SpannedExpr::app(ret, var, span);
            loop {
                if let Some(_) = self.consume(sym!(",")) {
                    let var = self.expr()?;
                    let span = Span::compose(&ret.span, &var.span);
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
                    Span::compose(&start_span, &end_token.span),
                ));
            } else if let Some(start_span) = self.consume(kwd!("mut")) {
                let ident = self.expect_ident()?;
                self.expect(sym!("="))?;
                let expr = self.expr()?;
                let end_token = self.expect(sym!(";"))?;
                prog.push(SpannedExpr::reassign(
                    ident,
                    expr,
                    Span::compose(&start_span, &end_token.span),
                ));
            } else if let Some(start_span) = self.consume(kwd!("enum")) {
                let ident = self.expect_ident()?;
                self.expect(sym!("{"))?;
                let defs = self.constructor_defs()?;
                self.expect(sym!("}"))?;
                let end_token = self.expect(sym!(";"))?;
                prog.push(SpannedExpr::enum_def(
                    ident,
                    defs,
                    Span::compose(&start_span, &end_token.span),
                ));
            } else {
                break;
            }
        }

        let ret = self.expr()?;
        let ret_span = if prog.is_empty() {
            ret.span
        } else {
            Span::compose(&prog[0].span, &ret.span)
        };
        Ok(SpannedExpr::program(prog, ret, ret_span))
    }

    fn constructor_defs(&mut self) -> Result<Vec<ConstructorDef>> {
        let def = self.constructor_def()?;
        let mut defs = vec![def];
        loop {
            if let Some(_) = self.consume(sym!(",")) {
                let def = self.constructor_def()?;
                defs.push(def);
            } else {
                break;
            }
        }
        Ok(defs)
    }

    fn constructor_def(&mut self) -> Result<ConstructorDef> {
        let name = self.expect_ident()?;
        let mut args = vec![];
        if let Some(_) = self.consume(sym!("(")) {
            let arg = self.parse_ty()?;
            args.push(arg);
            loop {
                if let Some(_) = self.consume(sym!(",")) {
                    let arg = self.parse_ty()?;
                    args.push(arg);
                } else {
                    break;
                }
            }
            self.expect(sym!(")"))?;
        }
        Ok(ConstructorDef { name, args })
    }

    fn arms(&mut self) -> Result<Vec<(Pattern, SpannedExpr)>> {
        let def = self.arm()?;
        let mut defs = vec![def];
        loop {
            if let Some(_) = self.consume(sym!(",")) {
                let def = self.arm()?;
                defs.push(def);
            } else {
                break;
            }
        }
        Ok(defs)
    }

    fn arm(&mut self) -> Result<(Pattern, SpannedExpr)> {
        let name = self.expect_ident()?;
        let mut vars = vec![];
        if let Some(_) = self.consume(sym!("(")) {
            let arg = self.expect_ident()?;
            vars.push(arg);
            loop {
                if let Some(_) = self.consume(sym!(",")) {
                    let arg = self.expect_ident()?;
                    vars.push(arg);
                } else {
                    break;
                }
            }
            self.expect(sym!(")"))?;
        }
        let pat = Pattern { name, vars };
        self.expect(sym!("=>"))?;
        let expr = self.expr()?;
        Ok((pat, expr))
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
                    "int" | "bool" | "string" => Ok(Type::constant(&val)),
                    _ => Err(ParseError::InvalidType { found: val, span }.into()),
                },
                Some(Token {
                    ttype: TokenType::Ident(val),
                    span: _,
                }) => Ok(Type::constant(&val)),

                Some(Token { ttype, span }) => Err(ParseError::ExpectedType {
                    found: ttype,
                    span: span,
                }
                .into()),
                None => Err(ParseError::UnexpectedEOF.into()),
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
