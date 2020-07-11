use std::iter::Peekable;
use std::str::Chars;

use crate::token::{Token, TokenType};

type Text<'t> = Peekable<Chars<'t>>;

pub struct Lexer {
    line: usize,

    token_len: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            line: 1,
            token_len: 0,
        }
    }

    /* NUMBER      → DIGIT+ ( "." DIGIT+ )? ;
    STRING         → "\"" <any char except "\"">* "\"" ;
    IDENTIFIER     → ALPHA ( ALPHA | DIGIT )* ;
    ALPHA          → "a" ... "z" | "A" ... "Z" | "_" ;
    DIGIT          → "0" ... "9" ; */

    pub fn next_token<'short: 'l, 'l>(&mut self, _text: &'l str) -> Option<Token<'l>> {
        let mut text: Text<'l> = _text.chars().peekable();

        self.token_len = 0;
        let typ: TokenType;

        match self.next(&mut text) {
            Some(c) => {
                match c {
                    '(' => typ = TokenType::LeftParen,
                    ')' => typ = TokenType::RightParen,
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
                        if self.advance_if(&mut text, '=') {
                            typ = TokenType::BangEqual
                        } else {
                            typ = TokenType::Bang
                        }
                    }
                    '=' => {
                        if self.advance_if(&mut text, '=') {
                            typ = TokenType::EqualEqual
                        } else {
                            typ = TokenType::Equal
                        }
                    }
                    '>' => {
                        if self.advance_if(&mut text, '=') {
                            typ = TokenType::GreaterEqual
                        } else {
                            typ = TokenType::Greater
                        }
                    }
                    '<' => {
                        if self.advance_if(&mut text, '=') {
                            typ = TokenType::LessEqual
                        } else {
                            typ = TokenType::Less
                        }
                    }
                    '\n' => {
                        typ = TokenType::Newline;
                        self.line += 1;
                    }
                    ' ' => {
                        typ = TokenType::Whitespace;
                        self.consume_while(&mut text, |c| c == ' ');
                    }
                    '\t' => {
                        typ = TokenType::Whitespace;
                        self.consume_while(&mut text, |c| c == '\t');
                    }
                    _ => unimplemented!(),
                };
                let lexeme: &str = &_text[..self.token_len];
                let ret: Token = Token::new(typ, lexeme, self.line);
                return Some(ret);
            }
            None => None,
        }
    }

    fn next(&mut self, text: &mut Text) -> Option<char> {
        self.token_len += 1;
        text.next()
    }

    fn advance_if(&mut self, text: &mut Text, ch: char) -> bool {
        match text.peek() {
            Some(c) if *c == ch => {
                self.next(text);
                true
            }
            _ => false,
        }
    }

    fn consume_while(&mut self, text: &mut Text, predicate: fn(char) -> bool) {
        loop {
            match text.peek() {
                Some(c) if predicate(*c) => {
                    self.next(text);
                }
                _ => return,
            }
        }
    }
}

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
