#![allow(dead_code, unused_imports)]

use std::iter::Peekable;

use crate::{
    bytecode::{opcodes, Chunk},
    error,
    lexer::{LexError, Lexer},
    token::{Token, TokenType},
    vm::RuntimeValue,
};

pub struct Compiler<'t> {
    text: &'t str,
    lexer: Peekable<Lexer<'t>>,

    pub bytecode: Chunk,

    had_error: bool,
    panic_mode: bool,
}

impl<'t> Compiler<'t> {
    pub fn new(text: &str) -> Compiler {
        Compiler {
            text,
            lexer: Lexer::new(text).into_iter().peekable(),

            bytecode: Chunk::new(),

            had_error: false,
            panic_mode: false,
        }
    }

    pub fn compile(&mut self) {
        self.expression();
        self.bytecode.add_opocode(opcodes::RETURN, 1);
    }

    pub fn dump_bytecode(&mut self) {
        self.bytecode.disassemble()
    }

    fn next_token(&mut self) -> Option<Token<'t>> {
        match self.lexer.next() {
            Some(t) => match t.typ {
                TokenType::Error(_) => panic!(),
                _ => return Some(t),
            },
            None => return None,
        }
    }

    fn peek_token(&mut self) -> Option<&Token> {
        match self.lexer.peek() {
            Some(t) => match t.typ {
                TokenType::Error(_) => panic!(),
                _ => return Some(t),
            },
            None => return None,
        }
    }

    fn expect_token(&mut self, expected_tok: TokenType) {
        match self.lexer.next() {
            Some(t) => {
                if expected_tok != t.typ {
                    error::report_parse_error();
                    panic!();
                }
            }
            _ => error::report_parse_error(),
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(parse_precedence::ASSIGNMENT);
    }

    fn parse_precedence(&mut self, prec: ParsePrecedence) {
        if let Some(t) = self.next_token() {
            self.prefix_rule(&t);

            loop {
                if let Some(peeked) = self.peek_token() {
                    if prec <= Compiler::precedence_rule(peeked.typ) {
                        // unwrap() is OK, since we already peeked
                        let t = self.next_token().unwrap();
                        self.infix_rule(&t);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.expect_token(TokenType::RightParen);
    }

    fn binary(&mut self, tok: &Token) {
        self.parse_precedence(Compiler::precedence_rule(tok.typ) + 1);

        match tok.typ {
            TokenType::Plus => self.bytecode.add_opocode(opcodes::ADD, tok.line),
            TokenType::Minus => self.bytecode.add_opocode(opcodes::SUBTRACT, tok.line),
            TokenType::Star => self.bytecode.add_opocode(opcodes::MULTIPLY, tok.line),
            TokenType::Slash => self.bytecode.add_opocode(opcodes::DIVIDE, tok.line),
            _ => unreachable!(),
        }
    }

    fn unary(&mut self, tok: &Token) {
        let operator_type = tok.typ;

        self.parse_precedence(parse_precedence::UNARY);

        match operator_type {
            TokenType::Minus => self.bytecode.add_opocode(opcodes::NEGATE, tok.line),
            _ => unreachable!(),
        }
    }

    fn number(&mut self, tok: &Token) {
        // TODO: handle double parsing error
        let num = tok.lexeme.parse::<f64>().unwrap();
        self.bytecode
            .add_constant(RuntimeValue::Number(num), tok.line);
    }

    fn literal(&mut self, tok: &Token) {
        match tok.typ {
            TokenType::Nil => self.bytecode.add_opocode(opcodes::NIL, tok.line),
            TokenType::True => self.bytecode.add_opocode(opcodes::TRUE, tok.line),
            TokenType::False => self.bytecode.add_opocode(opcodes::FALSE, tok.line),
            _ => unreachable!(),
        }
    }

    fn precedence_rule(typ: TokenType) -> ParsePrecedence {
        match typ {
            TokenType::Minus | TokenType::Plus => parse_precedence::TERM,
            TokenType::Slash | TokenType::Star => parse_precedence::FACTOR,
            _ => parse_precedence::NONE,
        }
    }

    fn prefix_rule(&mut self, tok: &Token) {
        match tok.typ {
            TokenType::LeftParen => self.grouping(),
            TokenType::Minus => self.unary(tok),
            TokenType::Number => self.number(tok),
            TokenType::Nil | TokenType::False | TokenType::True => self.literal(tok),
            _ => (),
        }
    }

    fn infix_rule(&mut self, tok: &Token) {
        match tok.typ {
            TokenType::Minus | TokenType::Plus | TokenType::Slash | TokenType::Star => {
                self.binary(tok);
            }
            _ => (),
        }
    }
}

type ParsePrecedence = u8;

mod parse_precedence {
    use super::ParsePrecedence;

    pub const NONE: ParsePrecedence = 0;
    pub const ASSIGNMENT: ParsePrecedence = 1;
    pub const OR: ParsePrecedence = 2;
    pub const AND: ParsePrecedence = 3;
    pub const EQUALITY: ParsePrecedence = 4;
    pub const COMPARISON: ParsePrecedence = 5;
    pub const TERM: ParsePrecedence = 6;
    pub const FACTOR: ParsePrecedence = 7;
    pub const UNARY: ParsePrecedence = 8;
    pub const CALL: ParsePrecedence = 9;
    pub const PRIMARY: ParsePrecedence = 10;
}
