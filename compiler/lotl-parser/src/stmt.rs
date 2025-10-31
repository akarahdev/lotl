use crate::parser::Parser;
use lotl_ast::stmt::AstStatement;

impl Parser {
    pub fn parse_stmt(&mut self) -> AstStatement {
        AstStatement::Drop {
            expr: self.parse_expr()
        }
    }

}
