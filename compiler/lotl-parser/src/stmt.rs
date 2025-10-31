use crate::errors::ExpectedKindFoundKind;
use crate::parser::Parser;
use lotl_ast::expr::AstExpr;
use lotl_ast::stmt::AstStatement;
use lotl_error::diagnostic::Diagnostic;
use lotl_token::TokenKind;

impl Parser {
    pub fn parse_stmt(&mut self) -> Option<AstStatement> {
        self.parse_expr().map(|expr| AstStatement::Drop { expr })
    }

    pub fn parse_expr(&mut self) -> Option<AstExpr> {
        self.parse_base_expr()
    }

    pub fn parse_base_expr(&mut self) -> Option<AstExpr> {
        let token = self.peek();
        match &token.kind {
            TokenKind::Numeric(num) => {
                self.next();
                Some(AstExpr::Numeric {
                    number: num.clone(),
                })
            }
            TokenKind::Ident(name) => {
                self.next();
                Some(AstExpr::Identifier { name: name.clone() })
            }
            found => {
                self.push_err(Diagnostic::new(
                    ExpectedKindFoundKind {
                        expected: &[
                            TokenKind::Numeric("".to_string()),
                            TokenKind::Ident("".to_string()),
                        ],
                        found: found.clone(),
                    },
                    token.location.clone(),
                ));
                None
            }
        }
    }
}
