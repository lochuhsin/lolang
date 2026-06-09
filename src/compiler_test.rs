#[cfg(test)]
mod test {
    use crate::chunk::Chunk;
    use crate::compiler::{declaration, expression, Compiler};
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    use crate::vm::disassemble_chunk;

    // #[test]
    // fn compile_string() {
    //     let str = String::from("\"abcde\"");
    //     let mut chunk = Chunk::default();
    //     compile(str, &mut chunk);
    //     disassemble_chunk(&chunk, "scan string");
    // }
    #[test]
    fn numeric_expression() {
        let s = String::from("1 + 2");
        let mut scanner = Scanner::new(s);
        let mut chunk = Chunk::default();
        let mut parser = Parser::new();
        let mut compiler = Compiler::default();
        parser.advance(&mut scanner);

        expression(&mut parser, &mut scanner, &mut chunk, &mut compiler);
        disassemble_chunk(&chunk, "scan string");
    }
    #[test]
    fn var_declaration() {
        let s = String::from("1 + 2");
        let mut scanner = Scanner::new(s);
        let mut chunk = Chunk::default();
        let mut parser = Parser::new();
        let mut compiler = Compiler::default();
        parser.advance(&mut scanner);

        declaration(&mut parser, &mut scanner, &mut chunk, &mut compiler);
        disassemble_chunk(&chunk, "scan string");
    }
}
