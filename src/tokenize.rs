use crate::span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Int(i64),
    Str(String),
    Symbol(String),
    Keyword(String),
    Type(String),
    Ident(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub span: Span,
}

impl Token {
    fn new(ttype: TokenType, line: u32, start: u32, end: u32) -> Token {
        Token {
            ttype,
            span: Span::new(line, start, end),
        }
    }
}

pub struct Tokenizer<'a> {
    input: &'a str,
    line: u32,
    col: u32,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            line: 0,
            col: 0,
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let parens: &str = "(){}[]";
        let keywords: Vec<&str> = vec![
            "true", "false", "if", "else", "let", "mut", "lambda", "enum", "match",
        ];
        let types: Vec<&str> = vec!["int", "bool", "string"];

        let mut ret = vec![];
        let mut program = self.input.chars().peekable();
        while let Some(ch) = program.next() {
            if ch == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }

            // 空白をスキップ
            if ch.is_whitespace() {
                continue;
            }

            // 数字のパース
            if ch.is_ascii_digit() {
                let mut numstr = ch.to_string();
                let start_col = self.col;
                while let Some(numch) = program.peek() {
                    if numch.is_ascii_digit() {
                        numstr.push(*numch);
                        let _ = program.next();
                        self.col += 1;
                    } else {
                        break;
                    }
                }
                let num = numstr.parse::<i64>().unwrap();
                ret.push(Token::new(
                    TokenType::Int(num),
                    self.line,
                    start_col,
                    self.col + 1,
                ));
                continue;
            }

            // 記号のパース
            if ch.is_ascii_punctuation() {
                let mut signs = ch.to_string();
                let start_col = self.col;

                // 文字列
                if ch == '\"' {
                    let mut string = "".to_string();
                    let start_col = self.col;
                    while let Some(ch) = program.next() {
                        if ch != '\"' {
                            // Escape
                            if ch == '\\' {
                                if let Some(ch2) = program.next() {
                                    match ch2 {
                                        'n' => string.push('\n'),
                                        '\\' => string.push('\\'),
                                        '\"' => string.push('\"'),
                                        _ => {}
                                    }
                                }
                            } else {
                                string.push(ch);
                            }
                        } else {
                            break;
                        }
                    }
                    ret.push(Token::new(
                        TokenType::Str(string),
                        self.line,
                        start_col,
                        self.col + 1,
                    ));
                    continue;
                }

                // コメント
                if ch == '/' {
                    if let Some('/') = program.peek() {
                        for ch in program.by_ref() {
                            if ch == '\n' {
                                break;
                            }
                        }
                        continue;
                    }
                }

                // カッコ
                if parens.contains(ch) {
                    ret.push(Token::new(
                        TokenType::Symbol(signs),
                        self.line,
                        start_col,
                        self.col + 1,
                    ));
                    continue;
                }

                // その他演算子
                while let Some(punctch) = program.peek() {
                    if punctch.is_ascii_punctuation() {
                        if parens.contains(*punctch) {
                            break;
                        }
                        signs.push(*punctch);
                        let _ = program.next();
                        self.col += 1;
                    } else {
                        break;
                    }
                }
                ret.push(Token::new(
                    TokenType::Symbol(signs),
                    self.line,
                    start_col,
                    self.col + 1,
                ));
                continue;
            }

            // 文字のパース
            if ch.is_ascii_alphabetic() {
                let mut ident = ch.to_string();
                let start_col = self.col;
                while let Some(identch) = program.peek() {
                    if identch.is_ascii_alphanumeric() {
                        ident.push(*identch);
                        let _ = program.next();
                        self.col += 1;
                    } else {
                        break;
                    }
                }

                let ttype = if keywords.contains(&ident.as_str()) {
                    TokenType::Keyword(ident)
                } else if types.contains(&ident.as_str()) {
                    TokenType::Type(ident)
                } else {
                    TokenType::Ident(ident)
                };
                ret.push(Token::new(ttype, self.line, start_col, self.col + 1));
                continue;
            }
        }

        ret
    }
}
