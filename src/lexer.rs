use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenType};

type Text<'t> = Peekable<Chars<'t>>;

// TODO: challenge - string interpolation

/*
 NUMBER      → DIGIT+ ( "." DIGIT+ )? ;
 STRING         → "\"" <any char except "\"">* "\"" ;
 IDENTIFIER     → ALPHA ( ALPHA | DIGIT )* ;
 ALPHA          → "a" ... "z" | "A" ... "Z" | "_" ;
 DIGIT          → "0" ... "9" ;
*/

pub struct Lexer<'t> {
    text: &'t str,

    line: usize,

    token_len: usize,
}

impl<'t> Lexer<'t> {
    pub fn new(text: &'t str) -> Lexer<'t> {
        Lexer {
            text,
            line: 1,
            token_len: 0,
        }
    }

    pub fn next_token(&mut self) -> Token<'t> {
        let mut text: Text<'t> = self.text.chars().peekable();

        self.token_len = 0;
        let typ: TokenType;

        match self.next_char(&mut text) {
            Some(c) => {
                match c {
                    c if Lexer::is_alpha(c) => {
                        typ = self.identifier_or_keyword(&mut text, c);
                    }
                    c if c.is_ascii_digit() => {
                        typ = TokenType::Number;
                        self.number(&mut text);
                    }
                    ' ' => {
                        typ = TokenType::Whitespace;
                        self.consume_while(&mut text, |c| c == ' ');
                    }
                    '\t' => {
                        typ = TokenType::Whitespace;
                        self.consume_while(&mut text, |c| c == '\t');
                    }
                    '\n' => {
                        typ = TokenType::Newline;
                        self.line += 1;
                    }
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
                    '\"' => typ = self.string(&mut text),
                    c => {
                        typ = TokenType::Error(LexError::InvalidCharacter);
                        // Assumes UTF-8 encoding, but whatever
                        self.token_len += c.len_utf8() - 1;
                    }
                };
                let lexeme: &str = &self.text[..self.token_len];
                self.text = &self.text[self.token_len..];
                Token::new(typ, lexeme, self.line)
            }
            None => Token::new(TokenType::Eof, "", self.line),
        }
    }

    #[inline]
    fn next_char(&mut self, text: &mut Text) -> Option<char> {
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
                self.next_char(text);
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
                    self.next_char(text);
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
            'f' => match self.next_char(text) {
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
            't' => match self.next_char(text) {
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
                    self.next_char(text);
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
                    self.next_char(text);
                    self.consume_while(text, |c| c.is_ascii_digit());
                }
            }
        }
    }

    #[inline]
    fn string(&mut self, text: &mut Text) -> TokenType {
        self.consume_while(text, |c| c != '"');
        match self.next_char(text) {
            Some('"') => TokenType::String,
            _ => {
                self.token_len -= 1;
                TokenType::Error(LexError::UnterminatedString)
            }
        }
    }

    #[inline]
    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LexError {
    InvalidCharacter,
    UnterminatedString,
}

#[cfg(test)]
mod test {
    #![allow(dead_code, unused_imports)]

    use crate::lexer::{Lexer, Token};
    use crate::token::TokenType::*;

    fn get_tokens(text: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(text);
        let mut tokens = Vec::new();

        loop {
            let tok = lexer.next_token();
            if tok.typ == Eof {
                break;
            }
            tokens.push(tok);
        }

        tokens
    }

    fn get_tokens_no_trivia(text: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(text);
        let mut tokens = Vec::new();

        loop {
            let tok = lexer.next_token();
            if tok.typ == Eof {
                break;
            }

            match tok.typ {
                Comment | Whitespace | Newline => continue,
                _ => tokens.push(tok),
            }
        }

        tokens
    }

    #[test]
    fn delimeters() {
        let tokens = get_tokens("(   ) {   } ,   . -   + ;   / *   ");

        let expected_tokens = vec![
            Token::new(LeftParen, "(", 1),
            Token::new(Whitespace, "   ", 1),
            Token::new(RightParen, ")", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(LeftBrace, "{", 1),
            Token::new(Whitespace, "   ", 1),
            Token::new(RightBrace, "}", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Comma, ",", 1),
            Token::new(Whitespace, "   ", 1),
            Token::new(Dot, ".", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Minus, "-", 1),
            Token::new(Whitespace, "   ", 1),
            Token::new(Plus, "+", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Semicolon, ";", 1),
            Token::new(Whitespace, "   ", 1),
            Token::new(Slash, "/", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Star, "*", 1),
            Token::new(Whitespace, "   ", 1),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn comparators() {
        let tokens = get_tokens("   ! != = == > >= < <=   \n");

        let expected_tokens = vec![
            Token::new(Whitespace, "   ", 1),
            Token::new(Bang, "!", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(BangEqual, "!=", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Equal, "=", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(EqualEqual, "==", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Greater, ">", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(GreaterEqual, ">=", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Less, "<", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(LessEqual, "<=", 1),
            Token::new(Whitespace, "   ", 1),
            Token::new(Newline, "\n", 2),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn identifiers() {
        let tokens = get_tokens(" x * 5 - foo   /bar + baz\n");

        let expected_tokens = vec![
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "x", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Star, "*", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Number, "5", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Minus, "-", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "foo", 1),
            Token::new(Whitespace, "   ", 1),
            Token::new(Slash, "/", 1),
            Token::new(Identifier, "bar", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Plus, "+", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "baz", 1),
            Token::new(Newline, "\n", 2),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn numbers() {
        let tokens = get_tokens(" x * 1.1/ 52.68 *654.7 - 7");

        let expected_tokens = vec![
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "x", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Star, "*", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Number, "1.1", 1),
            Token::new(Slash, "/", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Number, "52.68", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Star, "*", 1),
            Token::new(Number, "654.7", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Minus, "-", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Number, "7", 1),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn strings() {
        let tokens = get_tokens("  \"Hello World!\"  ");

        let expected_tokens = vec![
            Token::new(Whitespace, "  ", 1),
            Token::new(String, "\"Hello World!\"", 1),
            Token::new(Whitespace, "  ", 1),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn multiline_strings() {
        let tokens = get_tokens("  \"Hello World! \n after newline \"  ");

        let expected_tokens = vec![
            Token::new(Whitespace, "  ", 1),
            Token::new(String, "\"Hello World! \n after newline \"", 2),
            Token::new(Whitespace, "  ", 2),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn keywords() {
        let tokens = get_tokens(
            "and class else false for fun if nil or print return super this true var while",
        );

        let expected_tokens = vec![
            Token::new(And, "and", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Class, "class", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Else, "else", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(False, "false", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(For, "for", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Fun, "fun", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(If, "if", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Nil, "nil", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Or, "or", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Print, "print", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Return, "return", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Super, "super", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(This, "this", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(True, "true", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Var, "var", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(While, "while", 1),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn not_keywords_long() {
        let tokens = get_tokens(
            "andy classy elset falset forum funny iffy nilli ores prints returning superstition thisx truex variation whilex",
        );

        let expected_tokens = vec![
            Token::new(Identifier, "andy", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "classy", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "elset", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "falset", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "forum", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "funny", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "iffy", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "nilli", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "ores", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "prints", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "returning", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "superstition", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "thisx", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "truex", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "variation", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "whilex", 1),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn not_keywords_short() {
        let tokens = get_tokens("an clas els fals fo fu i ni o prin retur supe thi tru va whil");

        let expected_tokens = vec![
            Token::new(Identifier, "an", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "clas", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "els", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "fals", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "fo", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "fu", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "i", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "ni", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "o", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "prin", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "retur", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "supe", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "thi", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "tru", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "va", 1),
            Token::new(Whitespace, " ", 1),
            Token::new(Identifier, "whil", 1),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn for_loop() {
        let tokens = get_tokens_no_trivia(
            "for (var i = 1; i < 5; i = i + 1) {
        print i * i;
      }",
        );

        let expected_tokens = vec![
            Token::new(For, "for", 1),
            Token::new(LeftParen, "(", 1),
            Token::new(Var, "var", 1),
            Token::new(Identifier, "i", 1),
            Token::new(Equal, "=", 1),
            Token::new(Number, "1", 1),
            Token::new(Semicolon, ";", 1),
            Token::new(Identifier, "i", 1),
            Token::new(Less, "<", 1),
            Token::new(Number, "5", 1),
            Token::new(Semicolon, ";", 1),
            Token::new(Identifier, "i", 1),
            Token::new(Equal, "=", 1),
            Token::new(Identifier, "i", 1),
            Token::new(Plus, "+", 1),
            Token::new(Number, "1", 1),
            Token::new(RightParen, ")", 1),
            Token::new(LeftBrace, "{", 1),
            Token::new(Print, "print", 2),
            Token::new(Identifier, "i", 2),
            Token::new(Star, "*", 2),
            Token::new(Identifier, "i", 2),
            Token::new(Semicolon, ";", 2),
            Token::new(RightBrace, "}", 3),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn class() {
        let tokens = get_tokens_no_trivia(
            "class Duck {
        init(name) {
          this.name = name;
        }

        quack() {
          print this.name + \" quacks\";
        }
      }",
        );

        let expected_tokens = vec![
            Token::new(Class, "class", 1),
            Token::new(Identifier, "Duck", 1),
            Token::new(LeftBrace, "{", 1),
            Token::new(Identifier, "init", 2),
            Token::new(LeftParen, "(", 2),
            Token::new(Identifier, "name", 2),
            Token::new(RightParen, ")", 2),
            Token::new(LeftBrace, "{", 2),
            Token::new(This, "this", 3),
            Token::new(Dot, ".", 3),
            Token::new(Identifier, "name", 3),
            Token::new(Equal, "=", 3),
            Token::new(Identifier, "name", 3),
            Token::new(Semicolon, ";", 3),
            Token::new(RightBrace, "}", 4),
            Token::new(Identifier, "quack", 6),
            Token::new(LeftParen, "(", 6),
            Token::new(RightParen, ")", 6),
            Token::new(LeftBrace, "{", 6),
            Token::new(Print, "print", 7),
            Token::new(This, "this", 7),
            Token::new(Dot, ".", 7),
            Token::new(Identifier, "name", 7),
            Token::new(Plus, "+", 7),
            Token::new(String, "\" quacks\"", 7),
            Token::new(Semicolon, ";", 7),
            Token::new(RightBrace, "}", 8),
            Token::new(RightBrace, "}", 9),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn function() {
        let tokens = get_tokens_no_trivia(
            "fun make_adder(n) {
        fun adder(i) {
          return n + i;
        }
        return adder;
      }

      var add5 = make_adder(5);
      print add5(1);
      print add5(100);",
        );

        let expected_tokens = vec![
            Token::new(Fun, "fun", 1),
            Token::new(Identifier, "make_adder", 1),
            Token::new(LeftParen, "(", 1),
            Token::new(Identifier, "n", 1),
            Token::new(RightParen, ")", 1),
            Token::new(LeftBrace, "{", 1),
            Token::new(Fun, "fun", 2),
            Token::new(Identifier, "adder", 2),
            Token::new(LeftParen, "(", 2),
            Token::new(Identifier, "i", 2),
            Token::new(RightParen, ")", 2),
            Token::new(LeftBrace, "{", 2),
            Token::new(Return, "return", 3),
            Token::new(Identifier, "n", 3),
            Token::new(Plus, "+", 3),
            Token::new(Identifier, "i", 3),
            Token::new(Semicolon, ";", 3),
            Token::new(RightBrace, "}", 4),
            Token::new(Return, "return", 5),
            Token::new(Identifier, "adder", 5),
            Token::new(Semicolon, ";", 5),
            Token::new(RightBrace, "}", 6),
            Token::new(Var, "var", 8),
            Token::new(Identifier, "add5", 8),
            Token::new(Equal, "=", 8),
            Token::new(Identifier, "make_adder", 8),
            Token::new(LeftParen, "(", 8),
            Token::new(Number, "5", 8),
            Token::new(RightParen, ")", 8),
            Token::new(Semicolon, ";", 8),
            Token::new(Print, "print", 9),
            Token::new(Identifier, "add5", 9),
            Token::new(LeftParen, "(", 9),
            Token::new(Number, "1", 9),
            Token::new(RightParen, ")", 9),
            Token::new(Semicolon, ";", 9),
            Token::new(Print, "print", 10),
            Token::new(Identifier, "add5", 10),
            Token::new(LeftParen, "(", 10),
            Token::new(Number, "100", 10),
            Token::new(RightParen, ")", 10),
            Token::new(Semicolon, ";", 10),
        ];

        assert_eq!(tokens, expected_tokens);
    }
}
