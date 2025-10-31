use crate::errors::ExpectedKindFoundKind;
use crate::expect_kind;
use crate::parser::Parser;
use lotl_ast::defs::{AstDefinition, AstDefinitionId, AstDefinitionKind};
use lotl_error::diagnostic::Diagnostic;
use lotl_token::{TokenKind, TokenStream};
use uuid::Uuid;

impl Parser {
    pub fn parse_header(&mut self) -> Option<AstDefinition> {
        let kw_tok = self.peek();
        match &kw_tok.kind {
            TokenKind::EndOfStream => None,
            TokenKind::FuncKeyword => self.parse_function(),
            _ => {
                self.push_err(Diagnostic::new(
                    ExpectedKindFoundKind {
                        expected: &[TokenKind::FuncKeyword],
                        found: kw_tok.kind.clone(),
                    },
                    kw_tok.location.clone(),
                ));
                let _ = self.next();
                None
            }
        }
    }

    pub fn parse_function(&mut self) -> Option<AstDefinition> {
        let _kw_tok = self.next();
        let name_tok = self.next();

        let name = if let TokenKind::Ident(name) = name_tok.kind.clone() {
            name
        } else {
            self.push_err(Diagnostic::new(
                ExpectedKindFoundKind {
                    expected: &[TokenKind::Ident("".to_string())],
                    found: name_tok.kind.clone(),
                },
                name_tok.location.clone(),
            ));
            "__unnamed".to_string()
        };

        // parse generics of a functions
        let mut generics: Vec<String> = Vec::new();

        let generic_tok = self.peek();
        if let TokenKind::Brackets(generic_toks) = &generic_tok.kind {
            generics = self.parse_delimited_series(
                generic_toks.clone(),
                TokenKind::Comma,
                Parser::parse_generic_param,
            );
            self.next();
        }

        // parse the function's parameters
        let param_tok = self.peek();
        if let TokenKind::Parenthesis(_param_toks) = &param_tok.kind {
            self.next();
            // todo: parse parameters
        } else {
            let p = TokenKind::Parenthesis(TokenStream::empty());
            self.push_err(Diagnostic::new(
                ExpectedKindFoundKind {
                    expected: &[p],
                    found: param_tok.kind.clone(),
                },
                param_tok.location.clone(),
            ));
        };

        // now parse the return type
        let arrow_tok = self.next();
        expect_kind!(self, &arrow_tok, TokenKind::Arrow, &[TokenKind::Arrow]);

        let return_ty = self.parse_generic_type(generics.as_slice());

        let mut statements = None;
        if let TokenKind::Braces(block_tokens) = &self.peek().kind {
            self.next();
            statements = Some(
                self.parse_delimited_series(
                    block_tokens.clone(),
                    TokenKind::Semicolon,
                    Parser::parse_stmt,
                )
                .into_iter()
                .flatten()
                .collect(),
            )
        }
        Some(AstDefinition {
            kind: AstDefinitionKind::Function {
                name,
                parameters: vec![],
                generics,
                returns: return_ty,
                statements,
            },
            annotations: vec![],
            id: AstDefinitionId(Uuid::new_v4()),
        })
    }

    pub fn parse_generic_param(&mut self) -> String {
        let tok = self.next();
        let TokenKind::Ident(generic_type_name) = &tok.kind else {
            self.push_err(Diagnostic::new(
                ExpectedKindFoundKind {
                    expected: &[TokenKind::Ident("".to_string())],
                    found: tok.kind.clone(),
                },
                tok.location.clone(),
            ));
            return "".to_string();
        };
        generic_type_name.to_string()
    }
}
