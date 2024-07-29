#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Int(i64),
    Plus,
    Minus,
    Star,
    Div,
    LParen,
    RParen,
    LParenA,
    RParenA,
    Assign,
    Eq,
    Not,
    NotEq,
    LessEq,
    GreaterEq,
    Indent(String),
}

pub struct Tokenizer {
    input: String,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
        }
    }

    pub fn tokenize(&self) -> Vec<Token> {
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

            match ch {
                '+' => ret.push(Token::Plus),
                '-' => ret.push(Token::Minus),
                '*' => ret.push(Token::Star),
                '/' => ret.push(Token::Div),
                '(' => ret.push(Token::LParen),
                ')' => ret.push(Token::RParen),
                '<' => {
                    if let Some(nn) = program.peek() {
                        if *nn == '=' {
                            ret.push(Token::LessEq);
                            let _ = program.next();
                        } else {
                            ret.push(Token::LParenA);
                        }
                    }
                }
                '>' => {
                    if let Some(nn) = program.peek() {
                        if *nn == '=' {
                            ret.push(Token::GreaterEq);
                            let _ = program.next();
                        } else {
                            ret.push(Token::RParenA);
                        }
                    }
                }
                '=' => {
                    if let Some(nn) = program.peek() {
                        if *nn == '=' {
                            ret.push(Token::Eq);
                            let _ = program.next();
                        } else {
                            ret.push(Token::Assign);
                        }
                    }
                }
                '!' => {
                    if let Some(nn) = program.peek() {
                        if *nn == '=' {
                            ret.push(Token::NotEq);
                            let _ = program.next();
                        } else {
                            ret.push(Token::Not);
                        }
                    }
                }
                _ => panic!("unknown token"),
            }
        }

        ret
    }
}
