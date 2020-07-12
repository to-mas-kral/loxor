#![allow(dead_code, unused_imports)]

use crate::lexer::{Lexer, Token};
use crate::token::TokenType::*;

fn get_tokens(mut text: &str) -> Vec<Token> {
    let mut lexer = Lexer::new();
    let mut tokens = Vec::new();

    while let Some(tok) = lexer.next_token(text) {
        text = &text[tok.lexeme.len()..];
        tokens.push(tok);
    }

    tokens
}

fn get_tokens_no_trivia(mut text: &str) -> Vec<Token> {
    let mut lexer = Lexer::new();
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
fn comparators() {
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

#[test]
fn identifiers() {
    let tokens = get_tokens(" x * 5 - foo   /bar + baz\n");
    let mut expected_tokens = Vec::new();

    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "x", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Star, "*", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Number, "5", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Minus, "-", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "foo", 1));
    expected_tokens.push(Token::new(Whitespace, "   ", 1));
    expected_tokens.push(Token::new(Slash, "/", 1));
    expected_tokens.push(Token::new(Identifier, "bar", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Plus, "+", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "baz", 1));
    expected_tokens.push(Token::new(Newline, "\n", 2));

    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
}

#[test]
fn numbers() {
    let tokens = get_tokens(" x * 1.1/ 52.68 *654.7 - 7");
    let mut expected_tokens = Vec::new();

    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "x", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Star, "*", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Number, "1.1", 1));
    expected_tokens.push(Token::new(Slash, "/", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Number, "52.68", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Star, "*", 1));
    expected_tokens.push(Token::new(Number, "654.7", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Minus, "-", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Number, "7", 1));

    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
}

#[test]
fn strings() {
    let tokens = get_tokens("  \"Hello World!\"  ");
    let mut expected_tokens = Vec::new();

    expected_tokens.push(Token::new(Whitespace, "  ", 1));
    expected_tokens.push(Token::new(String, "\"Hello World!\"", 1));
    expected_tokens.push(Token::new(Whitespace, "  ", 1));

    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
}

#[test]
fn multiline_strings() {
    let tokens = get_tokens("  \"Hello World! \n after newline \"  ");
    let mut expected_tokens = Vec::new();

    expected_tokens.push(Token::new(Whitespace, "  ", 1));
    expected_tokens.push(Token::new(String, "\"Hello World! \n after newline \"", 2));
    expected_tokens.push(Token::new(Whitespace, "  ", 2));

    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
}

#[test]
fn keywords() {
    let tokens =
        get_tokens("and class else false for fun if nil or print return super this true var while");
    let mut expected_tokens = Vec::new();

    expected_tokens.push(Token::new(And, "and", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Class, "class", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Else, "else", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(False, "false", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(For, "for", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Fun, "fun", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(If, "if", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Nil, "nil", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Or, "or", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Print, "print", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Return, "return", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Super, "super", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(This, "this", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(True, "true", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Var, "var", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(While, "while", 1));

    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
}

#[test]
fn not_keywords_long() {
    let tokens = get_tokens(
            "andy classy elset falset forum funny iffy nilli ores prints returning superstition thisx truex variation whilex",
        );

    let mut expected_tokens = Vec::new();

    expected_tokens.push(Token::new(Identifier, "andy", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "classy", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "elset", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "falset", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "forum", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "funny", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "iffy", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "nilli", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "ores", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "prints", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "returning", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "superstition", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "thisx", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "truex", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "variation", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "whilex", 1));

    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
}

#[test]
fn not_keywords_short() {
    let tokens = get_tokens("an clas els fals fo fu i ni o prin retur supe thi tru va whil");

    let mut expected_tokens = Vec::new();

    expected_tokens.push(Token::new(Identifier, "an", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "clas", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "els", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "fals", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "fo", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "fu", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "i", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "ni", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "o", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "prin", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "retur", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "supe", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "thi", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "tru", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "va", 1));
    expected_tokens.push(Token::new(Whitespace, " ", 1));
    expected_tokens.push(Token::new(Identifier, "whil", 1));

    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
}

#[test]
fn for_loop() {
    let tokens = get_tokens_no_trivia(
        "for (var i = 1; i < 5; i = i + 1) {
        print i * i;
      }",
    );
    let mut expected_tokens = Vec::new();
    expected_tokens.push(Token::new(For, "for", 1));
    expected_tokens.push(Token::new(LeftParen, "(", 1));
    expected_tokens.push(Token::new(Var, "var", 1));
    expected_tokens.push(Token::new(Identifier, "i", 1));
    expected_tokens.push(Token::new(Equal, "=", 1));
    expected_tokens.push(Token::new(Number, "1", 1));
    expected_tokens.push(Token::new(Semicolon, ";", 1));
    expected_tokens.push(Token::new(Identifier, "i", 1));
    expected_tokens.push(Token::new(Less, "<", 1));
    expected_tokens.push(Token::new(Number, "5", 1));
    expected_tokens.push(Token::new(Semicolon, ";", 1));
    expected_tokens.push(Token::new(Identifier, "i", 1));
    expected_tokens.push(Token::new(Equal, "=", 1));
    expected_tokens.push(Token::new(Identifier, "i", 1));
    expected_tokens.push(Token::new(Plus, "+", 1));
    expected_tokens.push(Token::new(Number, "1", 1));
    expected_tokens.push(Token::new(RightParen, ")", 1));
    expected_tokens.push(Token::new(LeftBrace, "{", 1));
    expected_tokens.push(Token::new(Print, "print", 2));
    expected_tokens.push(Token::new(Identifier, "i", 2));
    expected_tokens.push(Token::new(Star, "*", 2));
    expected_tokens.push(Token::new(Identifier, "i", 2));
    expected_tokens.push(Token::new(Semicolon, ";", 2));
    expected_tokens.push(Token::new(RightBrace, "}", 3));

    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
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
    let mut expected_tokens = Vec::new();
    expected_tokens.push(Token::new(Class, "class", 1));
    expected_tokens.push(Token::new(Identifier, "Duck", 1));
    expected_tokens.push(Token::new(LeftBrace, "{", 1));
    expected_tokens.push(Token::new(Identifier, "init", 2));
    expected_tokens.push(Token::new(LeftParen, "(", 2));
    expected_tokens.push(Token::new(Identifier, "name", 2));
    expected_tokens.push(Token::new(RightParen, ")", 2));
    expected_tokens.push(Token::new(LeftBrace, "{", 2));
    expected_tokens.push(Token::new(This, "this", 3));
    expected_tokens.push(Token::new(Dot, ".", 3));
    expected_tokens.push(Token::new(Identifier, "name", 3));
    expected_tokens.push(Token::new(Equal, "=", 3));
    expected_tokens.push(Token::new(Identifier, "name", 3));
    expected_tokens.push(Token::new(Semicolon, ";", 3));
    expected_tokens.push(Token::new(RightBrace, "}", 4));
    expected_tokens.push(Token::new(Identifier, "quack", 6));
    expected_tokens.push(Token::new(LeftParen, "(", 6));
    expected_tokens.push(Token::new(RightParen, ")", 6));
    expected_tokens.push(Token::new(LeftBrace, "{", 6));
    expected_tokens.push(Token::new(Print, "print", 7));
    expected_tokens.push(Token::new(This, "this", 7));
    expected_tokens.push(Token::new(Dot, ".", 7));
    expected_tokens.push(Token::new(Identifier, "name", 7));
    expected_tokens.push(Token::new(Plus, "+", 7));
    expected_tokens.push(Token::new(String, "\" quacks\"", 7));
    expected_tokens.push(Token::new(Semicolon, ";", 7));
    expected_tokens.push(Token::new(RightBrace, "}", 8));
    expected_tokens.push(Token::new(RightBrace, "}", 9));

    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
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

    let mut expected_tokens = Vec::new();
    expected_tokens.push(Token::new(Fun, "fun", 1));
    expected_tokens.push(Token::new(Identifier, "make_adder", 1));
    expected_tokens.push(Token::new(LeftParen, "(", 1));
    expected_tokens.push(Token::new(Identifier, "n", 1));
    expected_tokens.push(Token::new(RightParen, ")", 1));
    expected_tokens.push(Token::new(LeftBrace, "{", 1));
    expected_tokens.push(Token::new(Fun, "fun", 2));
    expected_tokens.push(Token::new(Identifier, "adder", 2));
    expected_tokens.push(Token::new(LeftParen, "(", 2));
    expected_tokens.push(Token::new(Identifier, "i", 2));
    expected_tokens.push(Token::new(RightParen, ")", 2));
    expected_tokens.push(Token::new(LeftBrace, "{", 2));
    expected_tokens.push(Token::new(Return, "return", 3));
    expected_tokens.push(Token::new(Identifier, "n", 3));
    expected_tokens.push(Token::new(Plus, "+", 3));
    expected_tokens.push(Token::new(Identifier, "i", 3));
    expected_tokens.push(Token::new(Semicolon, ";", 3));
    expected_tokens.push(Token::new(RightBrace, "}", 4));
    expected_tokens.push(Token::new(Return, "return", 5));
    expected_tokens.push(Token::new(Identifier, "adder", 5));
    expected_tokens.push(Token::new(Semicolon, ";", 5));
    expected_tokens.push(Token::new(RightBrace, "}", 6));
    expected_tokens.push(Token::new(Var, "var", 8));
    expected_tokens.push(Token::new(Identifier, "add5", 8));
    expected_tokens.push(Token::new(Equal, "=", 8));
    expected_tokens.push(Token::new(Identifier, "make_adder", 8));
    expected_tokens.push(Token::new(LeftParen, "(", 8));
    expected_tokens.push(Token::new(Number, "5", 8));
    expected_tokens.push(Token::new(RightParen, ")", 8));
    expected_tokens.push(Token::new(Semicolon, ";", 8));
    expected_tokens.push(Token::new(Print, "print", 9));
    expected_tokens.push(Token::new(Identifier, "add5", 9));
    expected_tokens.push(Token::new(LeftParen, "(", 9));
    expected_tokens.push(Token::new(Number, "1", 9));
    expected_tokens.push(Token::new(RightParen, ")", 9));
    expected_tokens.push(Token::new(Semicolon, ";", 9));
    expected_tokens.push(Token::new(Print, "print", 10));
    expected_tokens.push(Token::new(Identifier, "add5", 10));
    expected_tokens.push(Token::new(LeftParen, "(", 10));
    expected_tokens.push(Token::new(Number, "100", 10));
    expected_tokens.push(Token::new(RightParen, ")", 10));
    expected_tokens.push(Token::new(Semicolon, ";", 10));

    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
}
