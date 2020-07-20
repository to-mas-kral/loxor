#![allow(dead_code, unused_imports)]

use crate::lexer::{Lexer, Token};
use crate::token::TokenType::*;

fn get_tokens(mut text: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(text);
    let mut tokens = Vec::new();

    while let Some(tok) = lexer.next_token(text) {
        text = &text[tok.lexeme.len()..];
        tokens.push(tok);
    }

    tokens
}

fn get_tokens_no_trivia(mut text: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(text);
    let mut tokens = Vec::new();

    while let Some(tok) = lexer.next_token(text) {
        text = &text[tok.lexeme.len()..];

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
    let tokens =
        get_tokens("and class else false for fun if nil or print return super this true var while");

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
