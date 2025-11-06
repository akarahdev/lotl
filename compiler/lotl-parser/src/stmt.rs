use crate::parser::Parser;
use lotl_ast::stmt::{AstStatement, StatementId};
use lotl_error::diagnostic::{Diagnostic, DiagnosticLevel};
use lotl_token::TokenKind;
use crate::errors::ExpectedKindFoundKind;

impl Parser {
    pub fn parse_stmt(&mut self) -> StatementId {
        match &self.peek().kind {
            TokenKind::ReturnKeyword => {
                self.next();
                let expr = self.parse_expr();
                self.stmts.register(|id| AstStatement::Returns { expr, id })
            }
            TokenKind::IfKeyword => {
                self.next();
                let cond = self.parse_expr();

                let mut if_true = Vec::new();
                if let TokenKind::Braces(block_tokens) = &self.peek().kind {
                    self.next();
                    if_true = self
                        .parse_delimited_series(
                            block_tokens.clone(),
                            TokenKind::Semicolon,
                            Parser::parse_stmt,
                        )
                        .into_iter()
                        .collect();
                }

                self.stmts.register(|id| AstStatement::If {
                    cond,
                    if_true,
                    otherwise: vec![],
                    id,
                })
            }
            TokenKind::ForKeyword => {
                self.next();
                let index_var = self.parse_ident();
                if let TokenKind::Colon = &self.peek().kind {
                    self.next();
                } else {
                   self.push_err(Diagnostic::new(
                       ExpectedKindFoundKind {
                           expected: &[TokenKind::Colon],
                           found: self.peek().kind.clone(),
                       },
                       self.peek().location.clone(),
                   ));
                }
                let iterable = self.parse_expr();
                let mut body = Vec::new();
                if let TokenKind::Braces(block_tokens) = &self.peek().kind {
                    self.next();
                    body = self
                        .parse_delimited_series(
                            block_tokens.clone(),
                            TokenKind::Semicolon,
                            Parser::parse_stmt,
                        );
                }

                self.stmts.register(|id| AstStatement::For {
                    index_var,
                    iterable,
                    body,
                    id,
                })
            }
            TokenKind::WhileKeyword => {
                self.next();
                let cond = self.parse_expr();
                let mut body = Vec::new();
                if let TokenKind::Braces(block_tokens) = &self.peek().kind {
                    self.next();
                    body = self
                        .parse_delimited_series(
                            block_tokens.clone(),
                            TokenKind::Semicolon,
                            Parser::parse_stmt,
                        );
                }
                self.stmts.register(|id| AstStatement::While { cond, body, id })
            }
            _ => {
                let expr = self.parse_expr();
                self.stmts.register(|id| AstStatement::Drop { expr, id })
            }
        }
    }
}
