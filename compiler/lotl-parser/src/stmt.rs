use crate::parser::Parser;
use lotl_ast::stmt::{AstStatement, StatementId};
use lotl_token::TokenKind;

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
            _ => {
                let expr = self.parse_expr();
                self.stmts.register(|id| AstStatement::Drop { expr, id })
            }
        }
    }
}
