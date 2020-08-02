use crate::{
    bytecode::{opcodes, Chunk},
    lexer::{LexError, Lexer},
    runtime_val::{RuntimeValue, StringObj},
    token::{Token, TokenType},
};

// TODO: challenge - better understand the Pratt parser
// TODO: challenge - implement the ternary operator

type CompileResult = Result<(), CompileErr>;

pub struct Compiler<'t> {
    text: &'t str,
    lexer: Lexer<'t>,
    peeked_tok: Option<Token<'t>>,

    pub bytecode: Chunk,

    last_error: CompileResult,
}

impl<'t> Compiler<'t> {
    pub fn new(text: &str) -> Compiler {
        Compiler {
            text,
            lexer: Lexer::new(text),
            peeked_tok: None,

            bytecode: Chunk::new(),

            last_error: Ok(()),
        }
    }

    pub fn dump_bytecode(&mut self) {
        self.bytecode.disassemble()
    }

    pub fn compile(&mut self) -> CompileResult {
        loop {
            match self.next_token() {
                Err(e) => {
                    self.last_error = Err(e);
                    self.synchronize();
                }
                Ok(tok) => {
                    if tok.typ == TokenType::Eof {
                        break;
                    } else {
                        if let Err(e) = self.declaration(&tok) {
                            self.last_error = Err(e);
                            self.synchronize();
                        }
                    }
                }
            }
        }

        self.bytecode.add_opocode(opcodes::RETURN, 0);
        self.last_error
    }

    fn declaration(&mut self, next_tok: &Token) -> CompileResult {
        match next_tok.typ {
            TokenType::Class => self.class_declaration(),
            TokenType::Fun => self.function_declaration(),
            TokenType::Var => self.variable_declaration(),
            _ => self.statement(&next_tok),
        }
    }

    fn class_declaration(&mut self) -> CompileResult {
        unimplemented!();
    }

    fn function_declaration(&mut self) -> CompileResult {
        unimplemented!();
    }

    fn variable_declaration(&mut self) -> CompileResult {
        unimplemented!();
    }

    fn statement(&mut self, tok: &Token) -> CompileResult {
        match tok.typ {
            TokenType::For => self.for_stmt(),
            TokenType::If => self.if_stmt(),
            TokenType::Print => self.print_stmt(&tok),
            TokenType::Return => self.return_stmt(),
            TokenType::While => self.while_stmt(),
            TokenType::LeftBrace => self.block_stmt(),
            typ if Compiler::is_expr_start(typ) => self.expr_stmt(tok),
            _ => {
                eprintln!(
                    "Parse error at line {}: expected declaration or statement, got: {}",
                    tok.line, tok.lexeme
                );
                Err(CompileErr::ExpectedDeclOrStmt)
            }
        }
    }

    fn expr_stmt(&mut self, tok: &Token) -> CompileResult {
        self.expression(tok)?;
        self.expect_token(TokenType::Semicolon)?;
        self.bytecode.add_opocode(opcodes::POP, tok.line);
        Ok(())
    }

    fn for_stmt(&mut self) -> CompileResult {
        unimplemented!()
    }

    fn if_stmt(&mut self) -> CompileResult {
        unimplemented!()
    }

    fn print_stmt(&mut self, print_tok: &Token) -> CompileResult {
        let first_expr_tok = self.next_token()?;
        self.expression(&first_expr_tok)?;
        self.expect_token(TokenType::Semicolon)?;
        self.bytecode.add_opocode(opcodes::PRINT, print_tok.line);
        Ok(())
    }

    fn return_stmt(&mut self) -> CompileResult {
        unimplemented!()
    }

    fn while_stmt(&mut self) -> CompileResult {
        unimplemented!()
    }

    fn block_stmt(&mut self) -> CompileResult {
        unimplemented!()
    }

    fn expression(&mut self, tok: &Token) -> CompileResult {
        if Compiler::is_expr_start(tok.typ) {
            self.parse_precedence(parse_precedence::ASSIGNMENT, tok)?;
            Ok(())
        } else {
            eprintln!(
                "Parse error at line {}: expected start of expression, got {:?}",
                tok.line, tok.typ
            );

            Err(CompileErr::ExpectedExpr)
        }
    }

    fn parse_precedence(&mut self, prec: ParsePrecedence, tok: &Token) -> CompileResult {
        self.prefix_rule(tok)?;

        loop {
            let peeked = self.peek_token();
            if prec <= Compiler::precedence_rule(peeked.typ) {
                // Here we can unwrap because we peeked already
                let t = self.next_token().unwrap();
                self.infix_rule(&t)?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn grouping(&mut self) -> CompileResult {
        let next_tok = self.next_token()?;
        self.expression(&next_tok)?;
        self.expect_token(TokenType::RightParen)?;
        Ok(())
    }

    fn binary(&mut self, tok: &Token) -> CompileResult {
        let next_tok = self.next_token()?;
        self.parse_precedence(Compiler::precedence_rule(tok.typ) + 1, &next_tok)?;

        match tok.typ {
            TokenType::Plus => self.bytecode.add_opocode(opcodes::ADD, tok.line),
            TokenType::Minus => self.bytecode.add_opocode(opcodes::SUBTRACT, tok.line),
            TokenType::Star => self.bytecode.add_opocode(opcodes::MULTIPLY, tok.line),
            TokenType::Slash => self.bytecode.add_opocode(opcodes::DIVIDE, tok.line),
            TokenType::BangEqual => {
                self.bytecode.add_opocode(opcodes::EQUAL, tok.line);
                self.bytecode.add_opocode(opcodes::NOT, tok.line);
            }
            TokenType::EqualEqual => self.bytecode.add_opocode(opcodes::EQUAL, tok.line),
            TokenType::Greater => self.bytecode.add_opocode(opcodes::GREATER, tok.line),
            TokenType::GreaterEqual => {
                self.bytecode.add_opocode(opcodes::LESS, tok.line);
                self.bytecode.add_opocode(opcodes::NOT, tok.line);
            }
            TokenType::Less => self.bytecode.add_opocode(opcodes::LESS, tok.line),
            TokenType::LessEqual => {
                self.bytecode.add_opocode(opcodes::GREATER, tok.line);
                self.bytecode.add_opocode(opcodes::NOT, tok.line);
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    fn unary(&mut self, tok: &Token) -> CompileResult {
        let operator_type = tok.typ;

        let next_tok = self.next_token()?;
        self.parse_precedence(parse_precedence::UNARY, &next_tok)?;

        match operator_type {
            TokenType::Minus => self.bytecode.add_opocode(opcodes::NEGATE, tok.line),
            TokenType::Bang => self.bytecode.add_opocode(opcodes::NOT, tok.line),
            _ => unreachable!(),
        }

        Ok(())
    }

    fn number(&mut self, tok: &Token) -> CompileResult {
        match tok.lexeme.parse::<f64>() {
            Ok(num) => {
                self.bytecode
                    .add_constant(RuntimeValue::Number(num), tok.line);
                Ok(())
            }
            Err(_) => {
                eprintln!(
                    "Parse error at line {}: couldn't parse {} as a number",
                    tok.line, tok.lexeme
                );
                Err(CompileErr::DoubleParse)
            }
        }
    }

    fn literal(&mut self, tok: &Token) {
        match tok.typ {
            TokenType::Nil => self.bytecode.add_opocode(opcodes::NIL, tok.line),
            TokenType::True => self.bytecode.add_opocode(opcodes::TRUE, tok.line),
            TokenType::False => self.bytecode.add_opocode(opcodes::FALSE, tok.line),
            _ => unreachable!(),
        }
    }

    fn string(&mut self, tok: &Token) {
        let slice = &tok
            .lexeme
            .strip_prefix("\"")
            .unwrap()
            .strip_suffix("\"")
            .unwrap();

        let string_ptr = StringObj::new(slice);
        let string_val = RuntimeValue::String(string_ptr);
        self.bytecode.add_constant(string_val, tok.line);
    }

    fn precedence_rule(typ: TokenType) -> ParsePrecedence {
        match typ {
            TokenType::Minus | TokenType::Plus => parse_precedence::TERM,
            TokenType::Slash | TokenType::Star => parse_precedence::FACTOR,
            TokenType::BangEqual | TokenType::EqualEqual => parse_precedence::EQUALITY,
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => parse_precedence::COMPARISON,
            _ => parse_precedence::NONE,
        }
    }

    fn prefix_rule(&mut self, tok: &Token) -> CompileResult {
        match tok.typ {
            TokenType::LeftParen => self.grouping()?,
            TokenType::Minus | TokenType::Bang => self.unary(tok)?,
            TokenType::Number => self.number(tok)?,
            TokenType::Nil | TokenType::False | TokenType::True => self.literal(tok),
            TokenType::String => self.string(tok),
            _ => (),
        };

        return Ok(());
    }

    fn infix_rule(&mut self, tok: &Token) -> CompileResult {
        match tok.typ {
            TokenType::Minus
            | TokenType::Plus
            | TokenType::Slash
            | TokenType::Star
            | TokenType::BangEqual
            | TokenType::EqualEqual
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => {
                self.binary(tok)?;
            }
            _ => (),
        };

        Ok(())
    }

    fn synchronize(&mut self) {
        loop {
            let tok = self.peek_token();
            match tok.typ {
                TokenType::Eof
                | TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => break,
                _ => {
                    self.next_token().ok();
                }
            }
        }
    }

    fn is_expr_start(typ: TokenType) -> bool {
        match typ {
            TokenType::Bang
            | TokenType::Minus
            | TokenType::True
            | TokenType::False
            | TokenType::Nil
            | TokenType::This
            | TokenType::Number
            | TokenType::String
            | TokenType::Identifier
            | TokenType::LeftParen
            | TokenType::Super => true,
            _ => false,
        }
    }

    fn next_token(&mut self) -> Result<Token<'t>, CompileErr> {
        match self.peeked_tok.take() {
            None => loop {
                let next_tok = self.lexer.next_token();
                match next_tok.typ {
                    TokenType::Whitespace | TokenType::Newline | TokenType::Comment => continue,
                    TokenType::Error(err) => {
                        eprintln!("Lexing error at line {}: {:?}", next_tok.line, err);

                        return Err(CompileErr::LexError);
                    }
                    _ => return Ok(next_tok),
                }
            },
            Some(t) => Ok(t),
        }
    }

    fn peek_token(&mut self) -> &Token {
        if self.peeked_tok.is_none() {
            loop {
                let next_tok = self.lexer.next_token();
                match next_tok.typ {
                    TokenType::Whitespace | TokenType::Newline | TokenType::Comment => continue,
                    _ => {
                        self.peeked_tok = Some(next_tok);
                        break;
                    }
                }
            }
        }

        self.peeked_tok.as_ref().unwrap()
    }

    fn expect_token(&mut self, expected_tok: TokenType) -> CompileResult {
        let tok = self.next_token()?;

        if expected_tok != tok.typ {
            eprintln!(
                "Parse error at line {}: exected {:?}, got {:?}",
                tok.line, expected_tok, tok.typ
            );
            return Err(CompileErr::UnexpectedToken);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompileErr {
    ExpectedDeclOrStmt,
    ExpectedExpr,
    UnexpectedToken,
    DoubleParse,
    LexError,
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

#[cfg(test)]
mod test {
    use crate::{
        bytecode::{opcodes::*, Chunk},
        compiler::Compiler,
        runtime_val::RuntimeValue,
    };

    fn get_chunk(text: &str) -> Chunk {
        let mut compiler = Compiler::new(text);
        compiler.compile();
        compiler.bytecode
    }

    #[test]
    fn arithmetic_expression_1() {
        let bytecode = get_chunk("1 + 2 * 5 - 6 / 3;");

        let expected_opcodes = vec![
            CONSTANT, 0, CONSTANT, 1, CONSTANT, 2, MULTIPLY, ADD, CONSTANT, 3, CONSTANT, 4, DIVIDE,
            SUBTRACT, POP, RETURN,
        ];

        let expected_constants = vec![1.0, 2.0, 5.0, 6.0, 3.0];

        assert_eq!(bytecode.code, expected_opcodes);

        for i in 0..bytecode.constants.len() {
            match bytecode.constants[i] {
                RuntimeValue::Number(n) => assert_eq!(n, expected_constants[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn airthmetic_expression_2() {
        let bytecode = get_chunk("0.3 - 1.2 - 100.1;");

        let expected_opcodes = vec![
            CONSTANT, 0, CONSTANT, 1, SUBTRACT, CONSTANT, 2, SUBTRACT, POP, RETURN,
        ];

        let expected_constants = vec![0.3, 1.2, 100.1];

        assert_eq!(bytecode.code, expected_opcodes);

        for i in 0..bytecode.constants.len() {
            match bytecode.constants[i] {
                RuntimeValue::Number(n) => assert_eq!(n, expected_constants[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn arithmetic_expression_3() {
        let bytecode = get_chunk("(1 + 2) * (5 - 6) / 3;");

        let expected_opcodes = vec![
            CONSTANT, 0, CONSTANT, 1, ADD, CONSTANT, 2, CONSTANT, 3, SUBTRACT, MULTIPLY, CONSTANT,
            4, DIVIDE, POP, RETURN,
        ];

        let expected_constants = vec![1.0, 2.0, 5.0, 6.0, 3.0];

        assert_eq!(bytecode.code, expected_opcodes);

        for i in 0..bytecode.constants.len() {
            match bytecode.constants[i] {
                RuntimeValue::Number(n) => assert_eq!(n, expected_constants[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn arithmetic_expression_4() {
        let bytecode = get_chunk("1 + 2 * 3 - 4 / -5;");

        let expected_opcodes = vec![
            CONSTANT, 0, CONSTANT, 1, CONSTANT, 2, MULTIPLY, ADD, CONSTANT, 3, CONSTANT, 4, NEGATE,
            DIVIDE, SUBTRACT, POP, RETURN,
        ];

        let expected_constants = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        assert_eq!(bytecode.code, expected_opcodes);

        for i in 0..bytecode.constants.len() {
            match bytecode.constants[i] {
                RuntimeValue::Number(n) => assert_eq!(n, expected_constants[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn comparison_expression_1() {
        let bytecode = get_chunk("(25 * 8) > -63;");

        let expected_opcodes = vec![
            CONSTANT, 0, CONSTANT, 1, MULTIPLY, CONSTANT, 2, NEGATE, GREATER, POP, RETURN,
        ];

        let expected_constants = vec![25.0, 8.0, 63.0];

        assert_eq!(bytecode.code, expected_opcodes);

        for i in 0..bytecode.constants.len() {
            match bytecode.constants[i] {
                RuntimeValue::Number(n) => assert_eq!(n, expected_constants[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn comparison_expression_2() {
        let bytecode = get_chunk("---5 >= --2 * 8 - 50;");

        let expected_opcodes = vec![
            CONSTANT, 0, NEGATE, NEGATE, NEGATE, CONSTANT, 1, NEGATE, NEGATE, CONSTANT, 2,
            MULTIPLY, CONSTANT, 3, SUBTRACT, LESS, NOT, POP, RETURN,
        ];

        let expected_constants = vec![5.0, 2.0, 8.0, 50.0];

        assert_eq!(bytecode.code, expected_opcodes);

        for i in 0..bytecode.constants.len() {
            match bytecode.constants[i] {
                RuntimeValue::Number(n) => assert_eq!(n, expected_constants[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn comparison_expression_3() {
        let bytecode = get_chunk("2 - 2 - -2 / 2 < -63;");

        let expected_opcodes = vec![
            CONSTANT, 0, CONSTANT, 1, SUBTRACT, CONSTANT, 2, NEGATE, CONSTANT, 3, DIVIDE, SUBTRACT,
            CONSTANT, 4, NEGATE, LESS, POP, RETURN,
        ];

        let expected_constants = vec![2.0, 2.0, 2.0, 2.0, 63.0];

        assert_eq!(bytecode.code, expected_opcodes);

        for i in 0..bytecode.constants.len() {
            match bytecode.constants[i] {
                RuntimeValue::Number(n) => assert_eq!(n, expected_constants[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn comparison_expression_4() {
        let bytecode = get_chunk("((((0)))) <= 10 * - 5 / 3;");

        let expected_opcodes = vec![
            CONSTANT, 0, CONSTANT, 1, CONSTANT, 2, NEGATE, MULTIPLY, CONSTANT, 3, DIVIDE, GREATER,
            NOT, POP, RETURN,
        ];

        let expected_constants = vec![0.0, 10.0, 5.0, 3.0];

        assert_eq!(bytecode.code, expected_opcodes);

        for i in 0..bytecode.constants.len() {
            match bytecode.constants[i] {
                RuntimeValue::Number(n) => assert_eq!(n, expected_constants[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn equality_expression_1() {
        let bytecode = get_chunk("0.5 * 10 - 3 < 5 == 50 >= 10;");

        let expected_opcodes = vec![
            CONSTANT, 0, CONSTANT, 1, MULTIPLY, CONSTANT, 2, SUBTRACT, CONSTANT, 3, LESS, CONSTANT,
            4, CONSTANT, 5, LESS, NOT, EQUAL, POP, RETURN,
        ];

        let expected_constants = vec![0.5, 10.0, 3.0, 5.0, 50.0, 10.0];

        assert_eq!(bytecode.code, expected_opcodes);

        for i in 0..bytecode.constants.len() {
            match bytecode.constants[i] {
                RuntimeValue::Number(n) => assert_eq!(n, expected_constants[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn equality_expression_2() {
        let bytecode = get_chunk("(0 > 10 == true) != (50 >= 10 == true);");

        let expected_opcodes = vec![
            CONSTANT, 0, CONSTANT, 1, GREATER, TRUE, EQUAL, CONSTANT, 2, CONSTANT, 3, LESS, NOT,
            TRUE, EQUAL, EQUAL, NOT, POP, RETURN,
        ];

        let expected_constants = vec![0.0, 10.0, 50.0, 10.0];

        assert_eq!(bytecode.code, expected_opcodes);

        for i in 0..bytecode.constants.len() {
            match bytecode.constants[i] {
                RuntimeValue::Number(n) => assert_eq!(n, expected_constants[i]),
                _ => assert!(false),
            }
        }
    }
}
