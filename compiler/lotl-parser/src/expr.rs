use lotl_ast::expr::{AstExpr, BinaryOperationKind};
use lotl_error::diagnostic::Diagnostic;
use lotl_token::TokenKind;
use crate::errors::ExpectedKindFoundKind;
use crate::parser::Parser;

impl Parser {
    pub fn parse_expr(&mut self) -> AstExpr {
        self.parse_term()
    }

    pub fn parse_term(&mut self) -> AstExpr {
        let mut base = self.parse_factor();
        loop {
            if self.peek().kind == TokenKind::Plus {
                self.next();
                base = AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Add,
                    lhs: Box::new(base),
                    rhs: Box::new(self.parse_factor())
                };
            } else if self.peek().kind == TokenKind::Minus {
                self.next();
                base = AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Subtract,
                    lhs: Box::new(base),
                    rhs: Box::new(self.parse_factor())
                }
            } else {
                break;
            }
        }
        base
    }

    pub fn parse_factor(&mut self) -> AstExpr {
        let mut base = self.parse_base_expr();

       loop {
            if self.peek().kind == TokenKind::Star {
                self.next();
                base = AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Multiply,
                    lhs: Box::new(base),
                    rhs: Box::new(self.parse_base_expr())
                };
            } else if self.peek().kind == TokenKind::Slash {
                self.next();
                base = AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Divide,
                    lhs: Box::new(base),
                    rhs: Box::new(self.parse_base_expr())
                }
            } else {
                break;
            }
        }
        base
    }

    pub fn parse_base_expr(&mut self) -> AstExpr {
        let token = self.peek();
        match &token.kind {
            TokenKind::Numeric(num) => {
                self.next();
                AstExpr::Numeric {
                    number: num.clone(),
                }
            }
            TokenKind::Ident(name) => {
                self.next();
                AstExpr::Identifier { name: name.clone() }
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
                AstExpr::Numeric { number: "".to_string() }
            }
        }
    }
}