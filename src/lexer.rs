use std::str::Chars;

use crate::token_type::TokenType;

pub struct Lexer {
    line: usize,
    token_start: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            line: 1,
            token_start: 0,
        }
    }

    /* NUMBER      → DIGIT+ ( "." DIGIT+ )? ;
    STRING         → "\"" <any char except "\"">* "\"" ;
    IDENTIFIER     → ALPHA ( ALPHA | DIGIT )* ;
    ALPHA          → "a" ... "z" | "A" ... "Z" | "_" ;
    DIGIT          → "0" ... "9" ; */

    pub fn next_token<'l>(&mut self, text: &'l str) -> Option<Token> {
        let mut cursor = Cursor::new(text);

        let typ: TokenType;

        match cursor.next() {
            Some(c) => {
                match c as char {
                    '(' => typ = TokenType::LeftParen,
                    ')' => typ = TokenType::LeftParen,
                    '{' => typ = TokenType::LeftBrace,
                    '}' => typ = TokenType::RightBrace,
                    ',' => typ = TokenType::Comma,
                    '.' => typ = TokenType::Dot,
                    '-' => typ = TokenType::Minus,
                    '+' => typ = TokenType::Plus,
                    ';' => typ = TokenType::Semicolon,
                    '/' => typ = TokenType::Slash,
                    '*' => typ = TokenType::Star,
                    '!' => {
                        if cursor.check(b'=') {
                            typ = TokenType::BangEqual
                        } else {
                            typ = TokenType::Bang
                        }
                    }
                    '=' => {
                        if cursor.check(b'=') {
                            typ = TokenType::EqualEqual
                        } else {
                            typ = TokenType::Equal
                        }
                    }
                    '>' => {
                        if cursor.check(b'=') {
                            typ = TokenType::GreaterEqual
                        } else {
                            typ = TokenType::Greater
                        }
                    }
                    '<' => {
                        if cursor.check(b'=') {
                            typ = TokenType::LessEqual
                        } else {
                            typ = TokenType::Less
                        }
                    }
                    _ => unimplemented!(),
                };
                let ret = Token::new(typ, self.token_start, cursor.token_len, self.line);
                self.token_start += cursor.token_len;
                Some(ret)
            }
            None => None,
        }
    }
}

struct Cursor<'c> {
    text: &'c [u8],
    text_len: usize,
    token_len: usize,
}

impl<'c> Cursor<'c> {
    pub fn new(text: &str) -> Cursor {
        Cursor {
            text: text.as_bytes(),
            text_len: text.len(),
            token_len: 0,
        }
    }

    pub fn next(&mut self) -> Option<u8> {
        match self.token_len + 1 < self.text_len {
            true => {
                self.token_len += 1;
                Some(self.text[self.token_len])
            }
            false => None,
        }
    }

    pub fn check(&mut self, ch: u8) -> bool {
        match self.token_len + 1 < self.text_len {
            true => {
                if ch == self.text[self.token_len + 1] {
                    self.token_len += 1;
                    true
                } else {
                    false
                }
            }
            false => false,
        }
    }

    /* pub fn peek_1(&mut self) -> Option<u8> {
        match self.token_len < self.text_len {
            true => Some(self.text[self.token_len + 1]),
            false => None,
        }
    } */
}

#[derive(Debug, PartialEq)]
pub struct Token {
    typ: TokenType,
    //lexeme: &'l str,
    start: usize,
    len: usize,
    line: usize,
}

impl Token {
    pub fn new(
        typ: TokenType,
        //lexeme: &'l str,
        start: usize,
        len: usize,
        line: usize,
    ) -> Token {
        Token {
            typ,
            start,
            len,
            line,
        }
    }
}

mod test {
    use crate::lexer::{Lexer, Token};

    fn get_tokens(mut text: &str) -> Vec<Token> {
        let mut lexer = Lexer::new();
        let mut tokens = Vec::new();

        while let Some(tok) = lexer.next_token(text) {
            text = &text[tok.len..];
            tokens.push(tok);
        }

        tokens
    }

    #[test]
    fn test_lexer_delims() {
        let tokens = get_tokens("(){},.-+;/*");
        let mut should_tokens = Vec::new();

        should_tokens.push("Token { typ: LeftParen, start: 0, len: 1, line: 1 }");
        should_tokens.push("Token { typ: LeftBrace, start: 1, len: 1, line: 1 }");
        should_tokens.push("Token { typ: RightBrace, start: 2, len: 1, line: 1 }");
        should_tokens.push("Token { typ: Comma, start: 3, len: 1, line: 1 }");
        should_tokens.push("Token { typ: Dot, start: 4, len: 1, line: 1 }");
        should_tokens.push("Token { typ: Minus, start: 5, len: 1, line: 1 }");
        should_tokens.push("Token { typ: Plus, start: 6, len: 1, line: 1 }");
        should_tokens.push("Token { typ: Semicolon, start: 7, len: 1, line: 1 }");
        should_tokens.push("Token { typ: Slash, start: 8, len: 1, line: 1 }");
        should_tokens.push("Token { typ: Star, start: 9, len: 1, line: 1 }");

        for i in 0..tokens.len() {
            assert_eq!(format!("{:?}", tokens[i]), should_tokens[i]);
        }
    }

    #[test]
    fn test_lexer_comparators() {
        let tokens = get_tokens("!,!=,=,==,>,>=,<,<=");
        let mut should_tokens = Vec::new();
        should_tokens.push("Token { typ: LeftParen, start: 0, len: 1, line: 1 }");

        for i in 0..tokens.len() {
            assert_eq!(format!("{:?}", tokens[i]), should_tokens[i]);
        }
    }
}
