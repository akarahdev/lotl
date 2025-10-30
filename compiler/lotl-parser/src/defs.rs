use crate::expect_kind;
use crate::parser::Parser;
use lotl_ast::defs::{AstDefinition, AstDefinitionId, AstDefinitionKind};
use lotl_error::diagnostic::{Diagnostic, DiagnosticLevel};
use lotl_token::TokenKind;
use uuid::Uuid;

impl Parser {
    pub fn parse_header(&mut self) -> Option<AstDefinition> {
        let kw_tok = self.peek();
        match &kw_tok.kind {
            TokenKind::EndOfStream => None,
            TokenKind::FuncKeyword => self.parse_function(),
            _ => {
                self.push_err(Diagnostic::new_dynamic(
                    format!("Expected `func`, got {:?}", kw_tok.kind),
                    DiagnosticLevel::Error,
                    kw_tok.location.clone(),
                ));
                let _ = self.next();
                None
            }
        }
    }

    pub fn parse_function(&mut self) -> Option<AstDefinition> {
        let kw_tok = self.next();
        let name_tok = self.next();

        expect_kind!(self, name_tok.clone(), TokenKind::Ident(name));

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
        let param_tok = self.next();
        let TokenKind::Parenthesis(_param_toks) = &param_tok.kind else {
            self.push_err(Diagnostic::new_dynamic(
                format!("Expected `ident`, got {:?}", kw_tok.kind),
                DiagnosticLevel::Error,
                param_tok.location.clone(),
            ));
            return None;
        };

        // now parse the return type
        let arrow_tok = self.next();
        expect_kind!(self, &arrow_tok, TokenKind::Arrow);

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
            self.push_err(Diagnostic::new_dynamic(
                format!("Expected `ident` as a generic type, got {:?}", tok.kind),
                DiagnosticLevel::Error,
                tok.location.clone(),
            ));
            return "".to_string();
        };
        generic_type_name.to_string()
    }
}
