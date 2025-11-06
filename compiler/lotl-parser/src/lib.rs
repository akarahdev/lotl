//! Holds the Lotl parser.
//! Exports a general `parse` method that transforms a file's TokenStream into a Vec<AstDefinition>
#![deny(missing_docs)]
mod defs;
mod errors;
mod expr;
mod parser;
mod util;

use crate::parser::Parser;
use lotl_ast::defs::AstDefinition;
use lotl_ast::expr::AstExpr;
use lotl_ast::graph::IdGraph;
use lotl_error::results::Results;
use lotl_token::{TokenKind, TokenStream};

/// Parses a TokenStream into a series of AstDefinitions.
pub fn parse(stream: TokenStream) -> Results<ParseResults> {
    let mut parser = Parser::new(stream);
    loop {
        if parser.peek().kind == TokenKind::EndOfStream {
            let errs = parser.get_errs();
            let results = ParseResults {
                definitions: parser.definitions,
                exprs: parser.exprs,
            };
            return Results::new(results, errs);
        }
        while parser.parse_header().is_some() {}
    }
}

/// Represents all nodes obtained from parsing a token stream.
#[derive(Debug)]
pub struct ParseResults {
    /// Contains the top-level headers
    pub definitions: IdGraph<AstDefinition>,
    /// Contains each expression in each statement
    pub exprs: IdGraph<AstExpr>,
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use lotl_error::file::SourceFile;
    use lotl_lexer::lex;

    #[test]
    fn empty_file() {
        let source = SourceFile::new("example.lotl", "");
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn simple_function() {
        let source = SourceFile::new("example.lotl", "func main() -> i32 { } ");
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn simple_content_function() {
        let source = SourceFile::new("example.lotl", "func main() -> i32 { 1; 2; 3; } ");
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn simple_generic_function() {
        let source = SourceFile::new("example.lotl", "func main[T]() -> T { } ");
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn bad_typeless_function() {
        let source = SourceFile::new("example.lotl", "func main() -> { }");
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 1);
    }

    #[test]
    fn bad_paramless_function() {
        let source = SourceFile::new("example.lotl", "func main -> i32 { }");
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 1);
    }

    #[test]
    fn binop_function() {
        let source = SourceFile::new(
            "example.lotl",
            "func main() -> i32 { 10 + 20 - 30 / 40 * 50; }",
        );
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn parenthesized_function() {
        let source = SourceFile::new("example.lotl", "func main() -> i32 { ((10) + (20)); }");
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn application_function() {
        let source = SourceFile::new("example.lotl", "func main() -> i32 { x.y; x[y]; x(y); }");
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn nested_application_function() {
        let source = SourceFile::new(
            "example.lotl",
            "func main() -> i32 { std::io::println(x); x[10].y[14](abc); }",
        );
        let ast = lex(source).bind(parse);
        eprintln!("{ast:#?}");
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn if_function() {
        let source = SourceFile::new(
            "example.lotl",
            "func main() -> i32 { if true { return 0; } }",
        );
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn for_function() {
        let source = SourceFile::new(
            "example.lotl",
            "func main() -> i32 { for x : list { std::io::println(x); }; }",
        );
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn while_function() {
        let source = SourceFile::new(
            "example.lotl",
            "func main() -> i32 { \
                while true { \
                    std::io::println(0); \
                }; \
            }",
        );
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    fn storing_function() {
        let source = SourceFile::new(
            "example.lotl",
            "func main() -> i32 { x = 10; y = 20; z = x + y; }",
        );
        let ast = lex(source).bind(parse);
        assert_eq!(ast.diagnostics.len(), 0);
    }
}
