#[cfg(test)]
mod test {
    use crate::{scanner::Scanner, tokens::TokenType};

    fn gen_tokens(scanner: &mut Scanner) -> Vec<TokenType> {
        let mut tokens = Vec::<TokenType>::new();
        loop {
            let t = scanner.scan_token();
            let token_type = *t.get_type();
            tokens.push(token_type);
            if token_type == TokenType::EOF {
                break;
            }
        }
        tokens
    }

    #[test]
    fn scan_string() {
        let mut scanner = Scanner::new("\"abcde\"".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::String, TokenType::EOF]);
    }
    #[test]
    fn scan_string_error() {
        let mut scanner = Scanner::new("\"abcde".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::ParseError, TokenType::EOF]);
    }
    #[test]
    fn scan_number() {
        let mut scanner = Scanner::new("123".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Number, TokenType::EOF]);

        let mut scanner = Scanner::new("123.456".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Number, TokenType::EOF]);

        // We don't allow .1234 or 1234. to be number, therefore ...
        let mut scanner = Scanner::new("123.".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(
            tokens,
            vec![TokenType::Number, TokenType::Dot, TokenType::EOF]
        );

        let mut scanner = Scanner::new(".123".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(
            tokens,
            vec![TokenType::Dot, TokenType::Number, TokenType::EOF]
        );
    }

    #[test]
    fn scan_bang() {
        let mut scanner = Scanner::new("!".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Bang, TokenType::EOF]);
    }
    #[test]
    fn scan_bangequal() {
        let mut scanner = Scanner::new("!=".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::BangEqual, TokenType::EOF]);
    }
    #[test]
    fn scan_equal() {
        let mut scanner = Scanner::new("=".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Equal, TokenType::EOF]);
    }
    #[test]
    fn scan_equalequal() {
        let mut scanner = Scanner::new("==".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::EqualEqual, TokenType::EOF]);
    }
    #[test]
    fn scan_greater() {
        let mut scanner = Scanner::new(">".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Greater, TokenType::EOF]);
    }
    #[test]
    fn scan_greaterequal() {
        let mut scanner = Scanner::new(">=".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::GreaterEqual, TokenType::EOF]);
    }
    #[test]
    fn scan_less() {
        let mut scanner = Scanner::new("<".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Less, TokenType::EOF]);
    }
    #[test]
    fn scan_lessequal() {
        let mut scanner = Scanner::new("<=".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::LessEqual, TokenType::EOF]);
    }
    #[test]
    fn scan_leftparen() {
        let mut scanner = Scanner::new("(".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::LeftParen, TokenType::EOF]);
    }
    #[test]
    fn scan_rightparen() {
        let mut scanner = Scanner::new(")".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::RightParen, TokenType::EOF]);
    }
    #[test]
    fn scan_leftbrace() {
        let mut scanner = Scanner::new("{".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::LeftBrace, TokenType::EOF]);
    }
    #[test]
    fn scan_rightbrace() {
        let mut scanner = Scanner::new("}".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::RightBrace, TokenType::EOF]);
    }
    #[test]
    fn scan_comma() {
        let mut scanner = Scanner::new(",".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Comma, TokenType::EOF]);
    }
    #[test]
    fn scan_dot() {
        let mut scanner = Scanner::new(".".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Dot, TokenType::EOF]);
    }
    #[test]
    fn scan_minus() {
        let mut scanner = Scanner::new("-".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Minus, TokenType::EOF]);
    }
    #[test]
    fn scan_plus() {
        let mut scanner = Scanner::new("+".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Plus, TokenType::EOF]);
    }
    #[test]
    fn scan_semicolon() {
        let mut scanner = Scanner::new(";".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Semicolon, TokenType::EOF]);
    }
    #[test]
    fn scan_slash() {
        let mut scanner = Scanner::new("/".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Slash, TokenType::EOF]);
    }
    #[test]
    fn scan_star() {
        let mut scanner = Scanner::new("*".to_string());
        let tokens = gen_tokens(&mut scanner);
        assert_eq!(tokens, vec![TokenType::Star, TokenType::EOF]);
    }
}
