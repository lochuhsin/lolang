#[cfg(test)]
mod test {
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    fn parse_string() {
        let s = String::from("\"abcde\"");
        let mut scanner = Scanner::new(s);
        let mut parser = Parser::new();
        parser.advance(&mut scanner);
        let obj = parser.current.unwrap();
        // TODO: add tests
    }
}
