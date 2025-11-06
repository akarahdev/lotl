use crate::parser::Parser;
use lotl_ast::ids::PureTag;
use lotl_ast::stmt::{AstStatement, StatementId};
use lotl_token::TokenKind;

impl Parser {
    pub fn parse_stmt(&mut self) -> AstStatement {
        match &self.peek().kind {
            TokenKind::ReturnKeyword => {
                self.next();
                let expr = self.parse_expr();
                AstStatement::Returns {
                    expr,
                    id: StatementId::make_new(),
                }
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

                AstStatement::If {
                    cond,
                    if_true,
                    otherwise: vec![],
                    id: StatementId::make_new(),
                }
            }
            _ => AstStatement::Drop {
                expr: self.parse_expr(),
                id: StatementId::make_new(),
            },
        }
    }
}
