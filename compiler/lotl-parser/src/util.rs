use crate::errors::ExpectedKindFoundKind;
use crate::parser::Parser;
use lotl_ast::types::AstType;
use lotl_error::diagnostic::Diagnostic;
use lotl_token::{TokenKind, TokenStream};

impl Parser {
    #[allow(unused)]
    pub fn parse_type(&mut self) -> AstType {
        self.parse_generic_type(&[])
    }

    pub fn parse_generic_type(&mut self, generics: &[String]) -> AstType {
        let type_ident = self.peek();
        let TokenKind::Ident(type_name) = &type_ident.kind else {
            self.push_err(Diagnostic::new(
                ExpectedKindFoundKind {
                    expected: &[TokenKind::Ident("".to_string())],
                    found: type_ident.kind.clone(),
                },
                type_ident.location.clone(),
            ));
            return AstType::Void;
        };
        self.next();
        if generics.contains(type_name) {
            return AstType::TypeVar(type_name.to_string());
        }
        match type_name.as_ref() {
            "i32" => AstType::Int32,
            "i64" => AstType::Int64,
            "f32" => AstType::Float32,
            "f64" => AstType::Float64,
            _ => AstType::Unresolved(type_name.to_string()),
        }
    }

    pub fn parse_single_stream<T, F: Fn(&mut Self) -> T>(&self, stream: TokenStream, func: F) -> T {
        let mut parser = Parser::new(stream);
        let output = func(&mut parser);
        for diag in parser.get_errs() {
            self.push_err(diag);
        }
        output
    }

    pub fn parse_delimited_series<T, F: Fn(&mut Self) -> T>(
        &self,
        stream: TokenStream,
        delimiter: TokenKind,
        func: F,
    ) -> Vec<T> {
        let mut parser = Parser::new(stream);
        let mut collection = Vec::new();

        if parser.peek().kind == TokenKind::EndOfStream {
            return collection;
        }

        loop {
            if parser.peek().kind == TokenKind::EndOfStream {
                for err in parser.get_errs() {
                    self.push_err(err);
                }
                return collection;
            }
            collection.push(func(&mut parser));
            let next = parser.next();
            if next.kind != delimiter && next.kind != TokenKind::EndOfStream {
                parser.push_err(Diagnostic::new(
                    ExpectedKindFoundKind {
                        expected: std::slice::from_ref(&delimiter),
                        found: next.kind.clone(),
                    },
                    next.location.clone(),
                ));
            }
        }
    }
}
