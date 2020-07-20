use crate::{lexer::LexError, token::Token};

mod source_map;

pub fn _report_lex_error(text: &str, token: Token, e: LexError) {
    match e {
        LexError::InvalidCharacter => {
            let mut error_msg = format!("error: invalid character: {}\n", token.lexeme);
            error_msg.push_str("--->\n");
            error_msg.push_str(_line(text, token.line).as_str());

            println!("{}", error_msg);
        }
        _ => unimplemented!(),
    }
}

pub fn report_parse_error() {}

fn _line(text: &str, line: usize) -> String {
    let mut lines = text.lines().skip(line - 1);
    format!("{} |    {}\n", line, lines.next().unwrap())
}
