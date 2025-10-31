#![deny(missing_docs)]

//! This crate exports a `lex` function you can use.

extern crate core;

mod err;
mod lexer;

use crate::lexer::Lexer;
use lotl_error::file::SourceFile;
use lotl_error::Results;
use lotl_token::TokenStream;

/// Converts a file into a token stream.
pub fn lex(file: SourceFile) -> Results<TokenStream> {
    Lexer::new(file.clone()).lex_repeatedly()
}

#[cfg(test)]
mod tests {
    use crate::lex;
    use lotl_error::file::SourceFile;
    use lotl_token::TokenKind;

    #[test]
    pub fn simple_arithmetic_tokenization() {
        let source = SourceFile::new("hello.lotl", "1+ 2 -3 *4/ 5");
        let tokens = lex(source);
        assert!(matches!(tokens.output[0].kind, TokenKind::Numeric(..)));
        assert!(matches!(tokens.output[1].kind, TokenKind::Plus));
        assert!(matches!(tokens.output[2].kind, TokenKind::Numeric(..)));
        assert!(matches!(tokens.output[3].kind, TokenKind::Minus));
        assert!(matches!(tokens.output[4].kind, TokenKind::Numeric(..)));
        assert!(matches!(tokens.output[5].kind, TokenKind::Star));
        assert!(matches!(tokens.output[6].kind, TokenKind::Numeric(..)));
        assert!(matches!(tokens.output[7].kind, TokenKind::Slash));
        assert!(matches!(tokens.output[8].kind, TokenKind::Numeric(..)));
        assert_eq!(tokens.diagnostics.len(), 0);
    }

    #[test]
    pub fn simple_ident_tokenization() {
        let source = SourceFile::new("hello.lotl", "hello abc 123");
        let tokens = lex(source);
        assert_eq!(tokens.output.len(), 4);
        assert!(matches!(tokens.output[0].kind, TokenKind::Ident(..)));
        assert!(matches!(tokens.output[1].kind, TokenKind::Ident(..)));
        assert!(matches!(tokens.output[2].kind, TokenKind::Numeric(..)));
        assert_eq!(tokens.diagnostics.len(), 0);
    }

    #[test]
    pub fn braces() {
        let source = SourceFile::new("hello.lotl", "{ hello }");
        let tokens = lex(source);
        assert_eq!(tokens.output.len(), 2);
        assert_eq!(tokens.diagnostics.len(), 0);
    }
    #[test]
    pub fn braces_errorful() {
        let source = SourceFile::new("hello.lotl", "{ hello");
        let tokens = lex(source);
        assert_eq!(tokens.output.len(), 2);
        assert_eq!(tokens.diagnostics.len(), 1);
    }

    #[test]
    pub fn realistic_example() {
        let source = SourceFile::new(
            "hello.lotl",
            r#"
        func main() -> i32 {
            return 10;
        }
        "#,
        );
        let tokens = lex(source);
        assert_eq!(tokens.output.len(), 7);
        assert_eq!(tokens.diagnostics.len(), 0);
    }
}
