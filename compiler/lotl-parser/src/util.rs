use crate::parser::Parser;
use lotl_ast::types::AstType;
use lotl_error::diagnostic::{Diagnostic, DiagnosticLevel};
use lotl_token::{TokenKind, TokenStream};

impl Parser {
    #[allow(unused)]
    pub fn parse_type(&mut self) -> AstType {
        self.parse_generic_type(&[])
    }

    pub fn parse_generic_type(&mut self, generics: &[String]) -> AstType {
        let type_ident = self.next();
        let TokenKind::Ident(type_name) = &type_ident.kind else {
            self.push_err(Diagnostic::new_static(
                "Not a valid type",
                DiagnosticLevel::Error,
                type_ident.location.clone(),
            ));
            return AstType::Void;
        };
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
                parser.push_err(Diagnostic::new_dynamic(
                    format!("Unexpected token {:?}, wanted {:?}", next.kind, delimiter),
                    DiagnosticLevel::Error,
                    next.location.clone(),
                ));
            }
        }
    }
}
