use crate::tokens::TokenType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Precedence {
    PrecNone = 0,
    PrecAssignment,
    PrecOr,
    PrecAnd,
    PrecEquality,
    PrecComparison,
    PrecTerm,
    PrecFactor,
    PrecUnary,
    PrecCall,
    PrecPrimary,
}

impl Precedence {
    pub fn from_usize(usize: usize) -> Precedence {
        match usize {
            0 => Precedence::PrecNone,
            1 => Precedence::PrecAssignment,
            2 => Precedence::PrecOr,
            3 => Precedence::PrecAnd,
            4 => Precedence::PrecEquality,
            5 => Precedence::PrecComparison,
            6 => Precedence::PrecTerm,
            7 => Precedence::PrecFactor,
            8 => Precedence::PrecUnary,
            9 => Precedence::PrecCall,
            10 => Precedence::PrecPrimary,
            _ => panic!("Invalid Precedence"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseFn {
    String,
    Literal,
    Number,
    Unary,
    Binary,
    Grouping,
    Variable,
    Null,
}

pub struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn get_rule(token_type: TokenType) -> Self {
        match token_type {
            TokenType::LeftParen => ParseRule {
                prefix: ParseFn::Grouping,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::RightParen => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::LeftBrace => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::RightBrace => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Comma => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Dot => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Minus => ParseRule {
                prefix: ParseFn::Unary,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecTerm,
            },
            TokenType::Plus => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecTerm,
            },
            TokenType::Semicolon => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Slash => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecFactor,
            },
            TokenType::Star => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecFactor,
            },
            TokenType::Bang => ParseRule {
                prefix: ParseFn::Unary,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::BangEqual => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecEquality,
            },
            TokenType::Equal => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::EqualEqual => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecEquality,
            },
            TokenType::Greater => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecComparison,
            },
            TokenType::GreaterEqual => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecComparison,
            },
            TokenType::Less => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecComparison,
            },
            TokenType::LessEqual => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Binary,
                precedence: Precedence::PrecComparison,
            },
            TokenType::Identifier => ParseRule {
                prefix: ParseFn::Variable,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::String => ParseRule {
                prefix: ParseFn::String,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Number => ParseRule {
                prefix: ParseFn::Number,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },

            TokenType::And => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Class => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Else => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::False => ParseRule {
                prefix: ParseFn::Literal,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::For => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Fun => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::If => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Nil => ParseRule {
                prefix: ParseFn::Literal,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Or => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Print => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Return => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Super => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::This => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::True => ParseRule {
                prefix: ParseFn::Literal,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::Var => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::While => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::ParseError => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            TokenType::EOF => ParseRule {
                prefix: ParseFn::Null,
                infix: ParseFn::Null,
                precedence: Precedence::PrecNone,
            },
            _ => panic!(
                "Unexpected token type occurred in parse rule, {}",
                token_type.as_string()
            ),
        }
    }
}
