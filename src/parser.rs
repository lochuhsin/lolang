use crate::scanner::Scanner;
use crate::tokens::{Token, TokenType};

#[derive(Default)]
pub struct Parser {
    pub current: Option<Token>,
    pub previous: Option<Token>,
    pub had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self, scanner: &mut Scanner) {
        self.previous = self.current.clone();

        loop {
            let token = scanner.scan_token();
            let token_type = *token.get_type();
            self.current = Some(token.clone()); // this is slow
            if token_type != TokenType::ParseError {
                break;
            }
            self.panic_mode = true;
            error_at(&token, &token.get_lexeme());
        }
    }
    pub fn consume(&mut self, token_type: TokenType, scanner: &mut Scanner, msg: &str) {
        let token = self
            .current
            .as_ref()
            .expect("self.current should not be none .... figure out why");

        if token.get_type() == &token_type {
            self.advance(scanner);
        } else {
            error_at(token, msg);
        }
    }
}

fn error_at(token: &Token, msg: &str) {
    print!("[line {}] Error", token.get_line());
    if token.get_type() == &TokenType::EOF {
        print!(" at end")
    } else if token.get_type() == &TokenType::ParseError {
    } else {
        print!(" at {}", token.get_lexeme())
    }
    println!(": {}", msg);
}
