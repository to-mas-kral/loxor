use crate::lexer::LexError;
#[derive(Debug, PartialEq)]
pub struct Token<'l> {
    pub typ: TokenType,
    pub lexeme: &'l str,
    pub line: usize,
}

impl<'l> Token<'l> {
    pub fn new(typ: TokenType, lexeme: &'l str, line: usize) -> Token<'l> {
        Token { typ, lexeme, line }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One Or Two Character Tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Comment,
    Whitespace,
    Newline,

    Error(LexError),
}
