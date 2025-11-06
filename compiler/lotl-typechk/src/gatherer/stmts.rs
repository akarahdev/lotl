use crate::gatherer::{TypeGatherer, TypedStack};
use lotl_ast::expr::AstExpr;
use lotl_ast::stmt::AstStatement;

impl<'a> TypeGatherer<'a> {
    pub fn infer_stmts(&mut self, stack: &mut TypedStack, statements: &[AstStatement]) {
        stack.push_frame();
        for statement in statements {
            match statement {
                AstStatement::Storage { ptr, value, .. } => {
                    if let AstExpr::Identifier { name, .. } = ptr {
                        let ty = self.infer_expr(stack, value);
                        stack.write_var(name, ty)
                    }
                }
                AstStatement::Drop { expr, .. } => {
                    self.infer_expr(stack, expr);
                }
                AstStatement::Returns { expr, .. } => {
                    self.infer_expr(stack, expr);
                }
                AstStatement::If {
                    cond,
                    if_true,
                    otherwise,
                    ..
                } => {
                    self.infer_expr(stack, cond);
                    stack.push_frame();
                    self.infer_stmts(stack, if_true.as_ref());
                    stack.pop_frame();
                    stack.push_frame();
                    self.infer_stmts(stack, otherwise.as_ref());
                    stack.pop_frame();
                }
            }
        }
        stack.pop_frame();
    }
}
