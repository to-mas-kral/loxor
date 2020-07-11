use std::str::Chars;

use crate::token::{Token, TokenType};

pub struct Lexer {
    line: usize,

    text_len: usize,
    token_len: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            line: 1,

            text_len: 0,
            token_len: 0,
        }
    }

    /* NUMBER      → DIGIT+ ( "." DIGIT+ )? ;
    STRING         → "\"" <any char except "\"">* "\"" ;
    IDENTIFIER     → ALPHA ( ALPHA | DIGIT )* ;
    ALPHA          → "a" ... "z" | "A" ... "Z" | "_" ;
    DIGIT          → "0" ... "9" ; */

    pub fn next_token<'short: 'l, 'l>(&mut self, _text: &'l str) -> Option<Token<'l>> {
        let text = _text.as_bytes();
        self.text_len = text.len();
        self.token_len = 0;

        let typ: TokenType;

        match self.next(text) {
            Some(c) => {
                match c {
                    b'(' => typ = TokenType::LeftParen,
                    b')' => typ = TokenType::RightParen,
                    b'{' => typ = TokenType::LeftBrace,
                    b'}' => typ = TokenType::RightBrace,
                    b',' => typ = TokenType::Comma,
                    b'.' => typ = TokenType::Dot,
                    b'-' => typ = TokenType::Minus,
                    b'+' => typ = TokenType::Plus,
                    b';' => typ = TokenType::Semicolon,
                    b'/' => typ = TokenType::Slash,
                    b'*' => typ = TokenType::Star,
                    b'!' => {
                        if self.check(text, b'=') {
                            typ = TokenType::BangEqual
                        } else {
                            typ = TokenType::Bang
                        }
                    }
                    b'=' => {
                        if self.check(text, b'=') {
                            typ = TokenType::EqualEqual
                        } else {
                            typ = TokenType::Equal
                        }
                    }
                    b'>' => {
                        if self.check(text, b'=') {
                            typ = TokenType::GreaterEqual
                        } else {
                            typ = TokenType::Greater
                        }
                    }
                    b'<' => {
                        if self.check(text, b'=') {
                            typ = TokenType::LessEqual
                        } else {
                            typ = TokenType::Less
                        }
                    }
                    b'\n' => {
                        typ = TokenType::Newline;
                        self.line += 1;
                    }
                    b' ' => {
                        typ = TokenType::Whitespace;
                        self.consume_while(text, |c| c == b' ');
                    }
                    b'\t' => {
                        typ = TokenType::Whitespace;
                        self.consume_while(text, |c| c == b'\t');
                    }
                    _ => unimplemented!(),
                };
                let lexeme: &str = &_text[..self.token_len];
                let ret: Token<'l> = Token::new(typ, lexeme, self.line);
                return Some(ret);
            }
            None => None,
        }
    }

    fn next(&mut self, text: &[u8]) -> Option<u8> {
        match self.token_len < self.text_len {
            true => {
                let ret = Some(text[self.token_len]);
                self.token_len += 1;
                ret
            }
            false => None,
        }
    }

    fn check(&mut self, text: &[u8], ch: u8) -> bool {
        match self.token_len < self.text_len {
            true => {
                if ch == text[self.token_len] {
                    self.token_len += 1;
                    true
                } else {
                    false
                }
            }
            false => false,
        }
    }

    fn consume_while(&mut self, text: &[u8], predicate: fn(u8) -> bool) {
        while self.token_len < self.text_len {
            let next_char = text[self.token_len];
            if predicate(next_char) {
                self.token_len += 1;
                continue;
            } else {
                break;
            }
        }
    }
}

/*  pub fn peek_1(&mut self) -> Option<u8> {
    match self.token_len < self.text_len {
        true => Some(self.text[self.token_len + 1]),
        false => None,
    }
}  */

mod test {
    use crate::lexer::{Lexer, Token};
    use crate::token::TokenType::*;

    fn get_tokens(mut text: &str) -> Vec<Token> {
        let mut lexer = Lexer::new();
        let mut tokens = Vec::new();

        while let Some(tok) = lexer.next_token(text) {
            text = &text[tok.lexeme.len()..];

            match tok.typ {
                /* TokenType::Whitespace | TokenType::Newline => continue, */
                _ => tokens.push(tok),
            }
        }

        tokens
    }

    #[test]
    fn test_lexer_delims() {
        let tokens = get_tokens("(   ) {   } ,   . -   + ;   / *   ");
        let mut expected_tokens = Vec::new();

        expected_tokens.push(Token::new(LeftParen, "(", 1));
        expected_tokens.push(Token::new(Whitespace, "   ", 1));
        expected_tokens.push(Token::new(RightParen, ")", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(LeftBrace, "{", 1));
        expected_tokens.push(Token::new(Whitespace, "   ", 1));
        expected_tokens.push(Token::new(RightBrace, "}", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(Comma, ",", 1));
        expected_tokens.push(Token::new(Whitespace, "   ", 1));
        expected_tokens.push(Token::new(Dot, ".", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(Minus, "-", 1));
        expected_tokens.push(Token::new(Whitespace, "   ", 1));
        expected_tokens.push(Token::new(Plus, "+", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(Semicolon, ";", 1));
        expected_tokens.push(Token::new(Whitespace, "   ", 1));
        expected_tokens.push(Token::new(Slash, "/", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(Star, "*", 1));
        expected_tokens.push(Token::new(Whitespace, "   ", 1));

        for i in 0..tokens.len() {
            assert_eq!(tokens[i], expected_tokens[i]);
        }
    }

    #[test]
    fn test_lexer_comparators() {
        let tokens = get_tokens("   ! != = == > >= < <=   \n");
        let mut expected_tokens = Vec::new();

        expected_tokens.push(Token::new(Whitespace, "   ", 1));
        expected_tokens.push(Token::new(Bang, "!", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(BangEqual, "!=", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(Equal, "=", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(EqualEqual, "==", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(Greater, ">", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(GreaterEqual, ">=", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(Less, "<", 1));
        expected_tokens.push(Token::new(Whitespace, " ", 1));
        expected_tokens.push(Token::new(LessEqual, "<=", 1));
        expected_tokens.push(Token::new(Whitespace, "   ", 1));
        expected_tokens.push(Token::new(Newline, "\n", 2));

        for i in 0..tokens.len() {
            assert_eq!(tokens[i], expected_tokens[i]);
        }
    }
}
