use crate::errors::ExpectedKindFoundKind;
use crate::parser::Parser;
use lotl_ast::expr::{AstExpr, BinaryOperationKind, ExprId};
use lotl_error::diagnostic::Diagnostic;
use lotl_token::TokenKind;

impl Parser {
    pub fn parse_expr(&mut self) -> ExprId {
        self.parse_flow()
    }

    pub fn parse_flow(&mut self) -> ExprId {
        match &self.peek().kind {
            TokenKind::Braces(block_tokens) => {
                self.next();
                let exprs = self
                    .parse_delimited_series(
                        block_tokens.clone(),
                        TokenKind::Semicolon,
                        Parser::parse_expr,
                    )
                    .into_iter()
                    .collect();
                self.exprs.register(|id| AstExpr::Block { exprs, id })
            }
            TokenKind::ReturnKeyword => {
                self.next();
                let expr = self.parse_expr();
                self.exprs.register(|id| AstExpr::Returns { expr, id })
            }
            TokenKind::IfKeyword => {
                self.next();
                let cond = self.parse_expr();
                let if_true = self.parse_expr();
                let otherwise = self.exprs.register(|id| AstExpr::Block { exprs: Vec::new(), id });
                self.exprs.register(|id| AstExpr::If {
                    cond,
                    if_true,
                    otherwise,
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
                let body = self.parse_expr();
                
                self.exprs.register(|id| AstExpr::For {
                    index_var,
                    iterable,
                    body,
                    id,
                })
            }
            TokenKind::WhileKeyword => {
                self.next();
                let cond = self.parse_expr();
                let body = self.parse_expr();
                self.exprs.register(|id| AstExpr::While { cond, body, id })
            }
            _ => {
                let expr = self.parse_term();
                if self.peek().kind == TokenKind::Equal {
                    self.next();
                    let value = self.parse_expr();
                    self.exprs.register(|id| AstExpr::Storage {
                        ptr: expr,
                        type_hint: None,
                        value,
                        id,
                    })
                } else {
                    expr
                }
            }
        }
    }
    pub fn parse_term(&mut self) -> ExprId {
        let mut lhs = self.parse_factor();
        loop {
            if self.peek().kind == TokenKind::Plus {
                let op_span = self.next().location.clone();
                let rhs = self.parse_factor();
                lhs = self.exprs.register(|id| AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Add,
                    lhs,
                    rhs,
                    op_span,
                    id,
                });
            } else if self.peek().kind == TokenKind::Minus {
                let op_span = self.next().location.clone();
                let rhs = self.parse_factor();
                lhs = self.exprs.register(|id| AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Subtract,
                    lhs,
                    rhs,
                    op_span,
                    id,
                });
            } else {
                break;
            }
        }
        lhs
    }

    pub fn parse_factor(&mut self) -> ExprId {
        let mut lhs = self.parse_applications();

        loop {
            if self.peek().kind == TokenKind::Star {
                let op_span = self.next().location.clone();
                let rhs = self.parse_applications();
                lhs = self.exprs.register(|id| AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Multiply,
                    lhs,
                    rhs,
                    op_span,
                    id,
                });
            } else if self.peek().kind == TokenKind::Slash {
                let op_span = self.next().location.clone();
                let rhs = self.parse_applications();
                lhs = self.exprs.register(|id| AstExpr::BinaryOperation {
                    op: BinaryOperationKind::Divide,
                    lhs,
                    rhs,
                    op_span,
                    id,
                });
            } else {
                break;
            }
        }
        lhs
    }

    pub fn parse_applications(&mut self) -> ExprId {
        let mut obj = self.parse_base_expr();

        loop {
            let lookahead = self.peek().clone();
            if let TokenKind::Parenthesis(stream) = &lookahead.kind {
                self.next();
                let parameters = self.parse_delimited_series(
                    stream.clone(),
                    TokenKind::Comma,
                    Parser::parse_expr,
                );
                obj = self.exprs.register(|id| AstExpr::Invocation {
                    obj,
                    parameters,
                    id,
                })
            } else if let TokenKind::Brackets(stream) = &lookahead.kind {
                let index = self.parse_single_stream(stream.clone(), Parser::parse_expr);
                self.next();
                obj = self
                    .exprs
                    .register(|id| AstExpr::Subscript { obj, index, id })
            } else if let TokenKind::Dot = lookahead.kind.clone() {
                self.next();
                let ident = self.parse_ident();
                obj = self.exprs.register(|id| AstExpr::FieldAccess {
                    obj,
                    field: ident,
                    id,
                })
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
                obj = self.exprs.register(|id| AstExpr::NamespaceAccess {
                    obj,
                    path: ident,
                    id,
                })
            } else {
                break;
            }
        }
        obj
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

    pub fn parse_base_expr(&mut self) -> ExprId {
        let token = self.peek();
        match &token.kind {
            TokenKind::Numeric(num) => {
                let span = self.next().location.clone();
                let number = num.clone();
                self.exprs
                    .register(|id| AstExpr::Numeric { number, span, id })
            }
            TokenKind::Ident(name) => {
                let span = self.next().location.clone();
                let name = name.clone();
                self.exprs
                    .register(|id| AstExpr::Identifier { name, span, id })
            }
            TokenKind::Parenthesis(inner) => {
                self.next();
                self.parse_single_stream(inner.clone(), Parser::parse_expr)
            }
            found => {
                let span = token.location.clone();
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
                self.exprs.register(|id| AstExpr::Numeric {
                    number: "".to_string(),
                    span,
                    id,
                })
            }
        }
    }
}
