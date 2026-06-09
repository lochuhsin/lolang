use std::fmt::Display;
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    // Single character Tokens
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
    Colon,
    QuestionMark,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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

    EOF,

    // Use for parsing, just ignore this token ...etc
    ParseError,
}

impl TokenType {
    pub fn as_string(&self) -> &'static str {
        match self {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Colon => ":",
            TokenType::Semicolon => ";",
            TokenType::Slash => "/",
            TokenType::Star => "*",
            TokenType::QuestionMark => "?",

            // One or two character tokens
            TokenType::Bang => "!",
            TokenType::BangEqual => "!=",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",

            // Literals
            TokenType::Identifier => "<Identifier>",
            TokenType::String => "<String>",
            TokenType::Number => "<Number>",

            // Keywords
            TokenType::And => "and",
            TokenType::Class => "class",
            TokenType::Else => "else",
            TokenType::False => "false",
            TokenType::Fun => "fun",
            TokenType::For => "for",
            TokenType::If => "if",
            TokenType::Nil => "nil",
            TokenType::Or => "or",
            TokenType::Print => "print",
            TokenType::Return => "return",
            TokenType::Super => "super",
            TokenType::This => "this",
            TokenType::True => "true",
            TokenType::Var => "var",
            TokenType::While => "while",

            // Parser thing
            TokenType::EOF => "EOF",
            TokenType::ParseError => "ParseError",
        }
    }

    pub fn keyword_to_token(s: &str) -> Option<TokenType> {
        match s {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),

            _ => None,
        }
    }

    pub fn keyword_to_token_dfa(s: &str) -> Option<TokenType> {
        /*
        automation finite state machine
         */
        match s.as_bytes()[0] {
            b'a' => TokenType::check_keyword_return(&s[1..3], "nd", TokenType::And),
            b'c' => TokenType::check_keyword_return(&s[1..5], "lass", TokenType::Class),
            b'e' => TokenType::check_keyword_return(&s[1..4], "lse", TokenType::Else),
            b'f' => match s.as_bytes()[1] {
                b'a' => TokenType::check_keyword_return(&s[2..5], "lse", TokenType::False),
                b'u' => TokenType::check_keyword_return(&s[2..3], "n", TokenType::Fun),
                b'o' => TokenType::check_keyword_return(&s[2..3], "r", TokenType::For),
                _ => None,
            },
            b'i' => TokenType::check_keyword_return(&s[1..2], "f", TokenType::If),
            b'n' => TokenType::check_keyword_return(&s[1..3], "il", TokenType::Nil),
            b'o' => TokenType::check_keyword_return(&s[1..2], "o", TokenType::Or),
            b'p' => TokenType::check_keyword_return(&s[1..5], "rint", TokenType::Print),
            b'r' => TokenType::check_keyword_return(&s[1..6], "eturn", TokenType::Return),
            b's' => TokenType::check_keyword_return(&s[1..5], "uper", TokenType::Super),
            b't' => match s.as_bytes()[1] {
                b'h' => TokenType::check_keyword_return(&s[2..4], "is", TokenType::This),
                b'r' => TokenType::check_keyword_return(&s[2..4], "ue", TokenType::True),
                _ => None,
            },
            b'v' => TokenType::check_keyword_return(&s[1..3], "ar", TokenType::Var),
            b'w' => TokenType::check_keyword_return(&s[1..5], "hile", TokenType::While),
            _ => None,
        }
    }

    fn check_keyword_return(s: &str, expect_s: &str, token_type: TokenType) -> Option<TokenType> {
        if s == expect_s {
            Some(token_type)
        } else {
            None
        }
    }
}
#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn get_line(&self) -> usize {
        self.line
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type.as_string(),
            self.lexeme,
            self.line
        )
    }
}
