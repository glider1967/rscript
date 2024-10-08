#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Int(i64),
    Symbol(String),
    Keyword(String),
    Type(String),
    Ident(String),
}

pub struct Tokenizer<'a> {
    input: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let parens: &str = "(){}[]";
        let keywords: Vec<&str> = vec!["true", "false", "if", "else", "let", "lambda"];
        let types: Vec<&str> = vec!["int", "bool"];

        let mut ret = vec![];
        let mut program = self.input.chars().peekable();
        while let Some(ch) = program.next() {
            if ch.is_whitespace() {
                continue;
            }

            if ch.is_ascii_digit() {
                let mut numstr = ch.to_string();
                while let Some(numch) = program.peek() {
                    if numch.is_ascii_digit() {
                        numstr.push(*numch);
                        let _ = program.next();
                    } else {
                        break;
                    }
                }
                let num = numstr.parse::<i64>().unwrap();
                ret.push(Token::Int(num));
                continue;
            }

            if ch.is_ascii_punctuation() {
                let mut signs = ch.to_string();

                if parens.contains(ch) {
                    ret.push(Token::Symbol(signs));
                    continue;
                }

                while let Some(punctch) = program.peek() {
                    if punctch.is_ascii_punctuation() {
                        signs.push(*punctch);
                        let _ = program.next();
                    } else {
                        break;
                    }
                }
                ret.push(Token::Symbol(signs));
                continue;
            }

            if ch.is_ascii_alphabetic() {
                let mut ident = ch.to_string();
                while let Some(identch) = program.peek() {
                    if identch.is_ascii_alphanumeric() {
                        ident.push(*identch);
                        let _ = program.next();
                    } else {
                        break;
                    }
                }

                if keywords.contains(&ident.as_str()) {
                    ret.push(Token::Keyword(ident))
                } else if types.contains(&ident.as_str()) {
                    ret.push(Token::Type(ident))
                } else {
                    ret.push(Token::Ident(ident));
                }
                continue;
            }
        }

        ret
    }
}
