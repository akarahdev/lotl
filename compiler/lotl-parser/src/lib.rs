//! Holds the Lotl parser.
//! Exports a general `parse` method that transforms a file's TokenStream into a Vec<AstDefinition>
#![deny(missing_docs)]

mod defs;
mod errors;
mod parser;
mod stmt;
mod util;
mod expr;

use crate::parser::Parser;
use lotl_ast::defs::AstDefinition;
use lotl_error::Results;
use lotl_token::{TokenKind, TokenStream};

/// Parses a TokenStream into a series of AstDefinitions.
pub fn parse(stream: TokenStream) -> Results<Vec<AstDefinition>> {
    let mut parser = Parser::new(stream);
    let mut defs = Vec::new();
    loop {
        if parser.peek().kind == TokenKind::EndOfStream {
            return Results::new(defs, parser.get_errs());
        }
        if let Some(def) = parser.parse_header() {
            defs.push(def);
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use lotl_error::file::SourceFile;
    use lotl_lexer::lex;

    #[test]
    fn empty_file() {
        let source = SourceFile::new("example.lotl", "");
        let ast = lex(source).and_then(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn simple_function() {
        let source = SourceFile::new("example.lotl", "func main() -> i32 { } ");
        let ast = lex(source).and_then(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn simple_content_function() {
        let source = SourceFile::new("example.lotl", "func main() -> i32 { 1; 2; 3; } ");
        let ast = lex(source).and_then(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn simple_generic_function() {
        let source = SourceFile::new("example.lotl", "func main[T]() -> T { } ");
        let ast = lex(source).and_then(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn bad_typeless_function() {
        let source = SourceFile::new("example.lotl", "func main() -> { }");
        let ast = lex(source).and_then(parse);
        assert_eq!(ast.diagnostics.len(), 1);
    }

    #[test]
    fn bad_paramless_function() {
        let source = SourceFile::new("example.lotl", "func main -> i32 { }");
        let ast = lex(source).and_then(parse);
        assert_eq!(ast.diagnostics.len(), 1);
    }



    #[test]
    fn binop_function() {
        let source = SourceFile::new("example.lotl", "func main() -> i32 { 10 + 20 - 30 * 40 / 50; }");
        let ast = lex(source).and_then(parse);
        eprintln!("{:#?}", ast);
        assert_eq!(ast.diagnostics.len(), 0);
    }
}
