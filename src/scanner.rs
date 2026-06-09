use crate::errors::error;
use crate::tokens::{Token, TokenType};
#[derive(Default)]
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn update_source(&mut self, source: String) {
        self.source = source;
        self.start = 0;
        self.current = 0;
        self.line = 1;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_chars();
        self.start = self.current;
        if self.is_at_end() {
            return Token::new(TokenType::EOF, String::new(), self.line);
        }
        let ch = self.advance();

        let token_type: TokenType = match ch {
            // single character tokens
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            // double character tokens
            '!' => {
                if self.match_sub_ch('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_sub_ch('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.match_sub_ch('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_sub_ch('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            // String Literals
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }
                if self.is_at_end() {
                    error(self.line, "Unterminated string literal");
                    TokenType::ParseError
                } else {
                    self.advance(); // closing string
                    TokenType::String
                }
            }
            _ => {
                // put somewhere else
                if ch.is_ascii_digit() {
                    self.match_number()
                } else if ch.is_alphabetic() || ch == '_' {
                    // Not sure why we need _
                    self.match_identifier()
                } else {
                    TokenType::ParseError
                }
            }
        };

        match token_type {
            TokenType::String => {
                self.add_token_with_bound(token_type, self.start + 1, self.current - 1)
            }
            TokenType::ParseError => Token::new(
                token_type,
                String::from(&self.source[self.start..self.current]),
                self.line,
            ),
            _ => self.add_token(token_type),
        }
    }

    fn skip_chars(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            };
        }
    }

    fn match_identifier(&mut self) -> TokenType {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        // NOTE: this is the regular string comparison way
        // if let Some(t) = TokenType::keyword_to_token(&self.source[self.start..self.current]) {
        //     t
        // } else {
        //     TokenType::Identifier
        // }

        /*
           abcdefg
           ^      ^
           |      |

           st    curr

           we match the string using DFA,
        */
        if let Some(t) = TokenType::keyword_to_token(&self.source[self.start..self.current]) {
            t
        } else {
            TokenType::Identifier
        }
    }

    fn match_number(&mut self) -> TokenType {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // fractional, as we don't allow "1234." to be a valid number
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        TokenType::Number
    }

    fn match_sub_ch(&mut self, expect: char) -> bool {
        /*
        if match consumes (advance), else not
        */
        if self.is_at_end() {
            return false;
        }
        if expect != self.source.as_bytes()[self.current] as char {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            String::from(&self.source[self.start..self.current]),
            self.line,
        )
    }

    fn add_token_with_bound(&self, token_type: TokenType, start: usize, end: usize) -> Token {
        Token::new(
            token_type,
            String::from(&self.source[start..end]),
            self.line,
        )
    }

    fn peek(&self) -> char {
        /*
           Lookahead
        */
        if self.is_at_end() {
            '\0'
        } else {
            self.source.as_bytes()[self.current] as char
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.as_bytes()[self.current + 1] as char
        }
    }

    fn advance(&mut self) -> char {
        /*
        I'm Assuming the inputs were always ASCII characters.
        Moreover, the entire logic could be implemented in more rusty way.
        Since there are self.source.chars().nth() or something like that.
        */
        let output = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        output
    }
}
