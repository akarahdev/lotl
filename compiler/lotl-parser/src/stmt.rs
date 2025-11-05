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
                    id: StatementId::make_new()
                }
            }
            _ => AstStatement::Drop {
                expr: self.parse_expr(),
                id: StatementId::make_new(),
            },
        }
    }
}
