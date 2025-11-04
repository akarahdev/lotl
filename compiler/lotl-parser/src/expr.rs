use crate::errors::ExpectedKindFoundKind;
use crate::parser::Parser;
use lotl_ast::expr::{AstExpr, BinaryOperationKind, ExprId};
use lotl_ast::ids::PureTag;
use lotl_error::diagnostic::Diagnostic;
use lotl_token::TokenKind;

impl Parser {
    pub fn parse_expr(&mut self) -> AstExpr {
        self.parse_term()
    }

    pub fn parse_term(&mut self) -> AstExpr {
        let mut base = self.parse_factor();
        loop {
            if self.peek().kind == TokenKind::Plus {
                let op_span = self.next().location.clone();
                base = AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Add,
                    lhs: Box::new(base),
                    rhs: Box::new(self.parse_factor()),
                    op_span,
                    id: ExprId::make_new(),
                };
            } else if self.peek().kind == TokenKind::Minus {
                let op_span = self.next().location.clone();
                base = AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Subtract,
                    lhs: Box::new(base),
                    rhs: Box::new(self.parse_factor()),
                    op_span,
                    id: ExprId::make_new(),
                }
            } else {
                break;
            }
        }
        base
    }

    pub fn parse_factor(&mut self) -> AstExpr {
        let mut base = self.parse_applications();

        loop {
            if self.peek().kind == TokenKind::Star {
                let op_span = self.next().location.clone();
                base = AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Multiply,
                    lhs: Box::new(base),
                    rhs: Box::new(self.parse_applications()),
                    op_span,
                    id: ExprId::make_new(),
                };
            } else if self.peek().kind == TokenKind::Slash {
                let op_span = self.next().location.clone();
                base = AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Divide,
                    lhs: Box::new(base),
                    rhs: Box::new(self.parse_applications()),
                    op_span,
                    id: ExprId::make_new(),
                }
            } else {
                break;
            }
        }
        base
    }

    pub fn parse_applications(&mut self) -> AstExpr {
        let mut base = self.parse_base_expr();

        loop {
            let lookahead = self.peek().clone();
            if let TokenKind::Parenthesis(stream) = &lookahead.kind {
                self.next();
                let parameters = self.parse_delimited_series(
                    stream.clone(),
                    TokenKind::Comma,
                    Parser::parse_expr,
                );
                base = AstExpr::Invocation {
                    func: Box::new(base),
                    parameters,
                    id: ExprId::make_new(),
                }
            } else if let TokenKind::Brackets(stream) = &lookahead.kind {
                let index = self.parse_single_stream(stream.clone(), Parser::parse_expr);
                self.next();
                base = AstExpr::Subscript {
                    obj: Box::new(base),
                    index: Box::new(index),
                    id: ExprId::make_new(),
                }
            } else if let TokenKind::Dot = lookahead.kind.clone() {
                self.next();
                let ident = self.parse_ident();
                base = AstExpr::FieldAccess {
                    obj: Box::new(base),
                    field: ident,
                    id: ExprId::make_new(),
                }
            } else if let TokenKind::Colon = lookahead.kind.clone() {
                self.next();
                let next_colon = self.peek();
                if let TokenKind::Colon = &next_colon.kind {
                    self.next();
                } else {
                    self.push_err(Diagnostic::new(
                        ExpectedKindFoundKind {
                            expected: &[TokenKind::Colon],
                            found: next_colon.kind.clone(),
                        },
                        next_colon.location.clone(),
                    ));
                }

                let ident = self.parse_ident();
                base = AstExpr::NamespaceAccess {
                    obj: Box::new(base),
                    path: ident,
                    id: ExprId::make_new(),
                }
            } else {
                break;
            }
        }
        base
    }

    pub fn parse_ident(&mut self) -> String {
        let token = self.peek();
        match &token.kind {
            TokenKind::Ident(name) => {
                self.next();
                name.clone()
            }
            found => {
                let span = self.peek().location.clone();
                self.push_err(Diagnostic::new(
                    ExpectedKindFoundKind {
                        expected: &[TokenKind::Ident("".to_string())],
                        found: found.clone(),
                    },
                    span,
                ));
                "".to_string()
            }
        }
    }

    pub fn parse_base_expr(&mut self) -> AstExpr {
        let token = self.peek();
        match &token.kind {
            TokenKind::Numeric(num) => {
                let span = self.next().location.clone();
                AstExpr::Numeric {
                    number: num.clone(),
                    span,
                    id: ExprId::make_new(),
                }
            }
            TokenKind::Ident(name) => {
                let span = self.next().location.clone();
                AstExpr::Identifier {
                    name: name.clone(),
                    span,
                    id: ExprId::make_new(),
                }
            }
            TokenKind::Parenthesis(inner) => {
                self.next();
                self.parse_single_stream(inner.clone(), Parser::parse_expr)
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
                AstExpr::Numeric {
                    number: "".to_string(),
                    span: token.location.clone(),
                    id: ExprId::make_new(),
                }
            }
        }
    }
}
