use crate::parser::Parser;
use lotl_ast::ids::PureTag;
use lotl_ast::stmt::{AstStatement, StatementId};

impl Parser {
    pub fn parse_stmt(&mut self) -> AstStatement {
        AstStatement::Drop {
            expr: self.parse_expr(),
            id: StatementId::make_new(),
        }
    }
}
