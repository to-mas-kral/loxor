use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenType};

mod test;

type Text<'t> = Peekable<Chars<'t>>;

/*
 NUMBER      → DIGIT+ ( "." DIGIT+ )? ;
 STRING         → "\"" <any char except "\"">* "\"" ;
 IDENTIFIER     → ALPHA ( ALPHA | DIGIT )* ;
 ALPHA          → "a" ... "z" | "A" ... "Z" | "_" ;
 DIGIT          → "0" ... "9" ;
*/

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
                    '/' => {
                        if self.advance_if(&mut text, '/') {
                            typ = TokenType::Comment;
                            self.consume_while(&mut text, |c| c != '\n');
                        } else {
                            typ = TokenType::Slash;
                        }
                    }
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
                    c if Lexer::is_alpha(c) => {
                        typ = self.identifier_or_keyword(&mut text, c);
                    }
                    c if c.is_ascii_digit() => {
                        typ = TokenType::Number;
                        self.number(&mut text);
                    }
                    '\"' => {
                        typ = TokenType::String;
                        self.string(&mut text);
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

    #[inline]
    fn next(&mut self, text: &mut Text) -> Option<char> {
        self.token_len += 1;
        text.next()
    }

    #[inline]
    fn peek_2(&mut self, _text: &mut Text) -> Option<char> {
        let mut cloned = _text.clone();

        cloned.next()?;
        cloned.next()
    }

    #[inline]
    fn advance_if(&mut self, text: &mut Text, ch: char) -> bool {
        match text.peek() {
            Some(c) if *c == ch => {
                self.next(text);
                true
            }
            _ => false,
        }
    }

    #[inline]
    fn consume_while(&mut self, text: &mut Text, predicate: fn(char) -> bool) {
        loop {
            match text.peek() {
                Some(c) if predicate(*c) => {
                    if *c == '\n' {
                        self.line += 1;
                    }
                    self.next(text);
                }
                _ => return,
            }
        }
    }

    #[inline]
    fn identifier_or_keyword(&mut self, text: &mut Text, first: char) -> TokenType {
        match first {
            'a' => self.check_keyword(text, "nd", TokenType::And),
            'c' => self.check_keyword(text, "lass", TokenType::Class),
            'e' => self.check_keyword(text, "lse", TokenType::Else),
            'f' => match self.next(text) {
                Some(c) => match c {
                    'a' => self.check_keyword(text, "lse", TokenType::False),
                    'o' => self.check_keyword(text, "r", TokenType::For),
                    'u' => self.check_keyword(text, "n", TokenType::Fun),
                    _ => self.finish_identifier(text),
                },
                _ => self.finish_identifier(text),
            },
            'i' => self.check_keyword(text, "f", TokenType::If),
            'n' => self.check_keyword(text, "il", TokenType::Nil),
            'o' => self.check_keyword(text, "r", TokenType::Or),
            'p' => self.check_keyword(text, "rint", TokenType::Print),
            'r' => self.check_keyword(text, "eturn", TokenType::Return),
            's' => self.check_keyword(text, "uper", TokenType::Super),
            't' => match self.next(text) {
                Some(c) => match c {
                    'h' => self.check_keyword(text, "is", TokenType::This),
                    'r' => self.check_keyword(text, "ue", TokenType::True),
                    _ => self.finish_identifier(text),
                },
                _ => self.finish_identifier(text),
            },
            'v' => self.check_keyword(text, "ar", TokenType::Var),
            'w' => self.check_keyword(text, "hile", TokenType::While),
            _ => self.finish_identifier(text),
        }
    }

    #[inline]
    fn finish_identifier(&mut self, text: &mut Text) -> TokenType {
        self.consume_while(text, |c| c.is_ascii_alphanumeric() || c == '_');
        TokenType::Identifier
    }

    #[inline]
    fn check_keyword(&mut self, text: &mut Text, keyword: &str, typ: TokenType) -> TokenType {
        for c in keyword.chars() {
            match text.peek() {
                Some(p) if *p == c => {
                    self.next(text);
                }
                _ => return self.finish_identifier(text),
            }
        }

        match text.peek() {
            Some(c) if c.is_ascii_alphanumeric() || *c == '_' => self.finish_identifier(text),
            _ => typ,
        }
    }

    #[inline]
    fn number(&mut self, text: &mut Text) {
        self.consume_while(text, |c| c.is_ascii_digit());

        if let Some('.') = text.peek() {
            if let Some(c) = self.peek_2(text) {
                if c.is_ascii_digit() {
                    self.next(text);
                    self.consume_while(text, |c| c.is_ascii_digit());
                }
            }
        }
    }

    #[inline]
    fn string(&mut self, text: &mut Text) {
        self.consume_while(text, |c| c != '"');
        match self.next(text) {
            Some('"') => return,
            // TODO: error reporting
            _ => panic!("unterminated string"),
        }
    }

    #[inline]
    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }
}
