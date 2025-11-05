use crate::errors::{TypeMismatch, VariableNotFound};
use crate::gatherer::{TypeGatherer, TypedStack};
use lotl_ast::expr::AstExpr;
use lotl_ast::ids::Tagged;
use lotl_ast::types::AstType;
use lotl_error::diagnostic::Diagnostic;

impl<'a> TypeGatherer<'a> {
    pub fn infer_expr(&mut self, stack: &mut TypedStack, expr: &AstExpr) -> AstType {
        if let Some(ty) = self.ctx.type_of_expr(expr.id()) {
            return ty.clone();
        }
        match expr {
            AstExpr::Identifier { name, span, .. } => match stack.lookup_var(name) {
                Some(ty) => ty.clone(),
                None => {
                    self.ctx.diagnostics.push(Diagnostic::new(
                        VariableNotFound {
                            name: name.to_string(),
                        },
                        span.clone(),
                    ));
                    AstType::Void
                }
            },
            AstExpr::Numeric { number, .. } => {
                let mut ty = AstType::Int64;
                if number.contains('.') {
                    ty = AstType::Float64;
                }
                self.ctx.record_expr(expr.id(), ty.clone());
                ty
            }
            AstExpr::BinaryOperation { lhs, rhs, op_span, .. } => {
                let lhs_ty = self.infer_expr(stack, lhs);
                self.ctx.record_expr(expr.id(), lhs_ty.clone());
                let rhs_ty = self.infer_expr(stack, rhs);
                if lhs_ty != rhs_ty {
                    self.ctx.diagnostics.push(Diagnostic::new(
                        TypeMismatch {
                            expected: std::slice::from_ref(&lhs_ty),
                            found: &rhs_ty
                        },
                        op_span.clone(),
                    ))
                }
                lhs_ty
            }
            AstExpr::UnaryOperation { expr, .. } => {
                let ty = self.infer_expr(stack, expr);
                self.ctx.record_expr(expr.id(), ty.clone());
                ty
            }
            AstExpr::Invocation { .. } => AstType::Void,
            AstExpr::FieldAccess { .. } => AstType::Void,
            AstExpr::NamespaceAccess { .. } => AstType::Void,
            AstExpr::Subscript { .. } => AstType::Void,
        }
    }
}
