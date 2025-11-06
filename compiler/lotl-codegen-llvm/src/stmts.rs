use lotl_ast::expr::{AstExpr, BinaryOperationKind};
use lotl_ast::stmt::AstStatement;
use lotl_llvm_api::instruction::{BasicBlock, SharedBasicBlock};
use lotl_llvm_api::types::{Type, Types};
use lotl_llvm_api::value::{Value, Values};
use lotl_typechk::context::TyContext;

pub struct CodegenContext<'a> {
    pub types: &'a TyContext,
    pub block: SharedBasicBlock,
}

impl<'a> CodegenContext<'a> {
    pub fn stmts_to_bb(&mut self, stmts: &[AstStatement]) {
        for stmt in stmts {
            self.stmt_to_bb(stmt);
        }
    }

    pub fn stmt_to_bb(&mut self, stmt: &AstStatement) {
        match stmt {
            AstStatement::Storage { ptr, value, .. } => {
                let target = self.expr_to_bb_ptr(ptr);
                let val = self.expr_to_bb_value(value);
                self.block.store(target.1, val);
            }
            AstStatement::Drop { expr, .. } => {
                self.expr_to_bb_value(expr);
            }
            AstStatement::Returns { expr, .. } => {
                let value = self.expr_to_bb_value(expr);
                self.block.ret(value);
            }
            AstStatement::If {
                cond,
                if_true,
                otherwise,
                ..
            } => {
                let cond_val = self.expr_to_bb_value(cond);
                let cond_trunc = self.block.trunc(cond_val, Types::integer(1));
                let next = BasicBlock::child(&self.block);
                let branches = self.block.br_if_returning(cond_trunc);
                
                self.block = branches.0;
                self.stmts_to_bb(if_true);
                self.block.goto(&next);
                self.block = branches.1;
                self.stmts_to_bb(otherwise);
                self.block.goto(&next);
                self.block = next;
            }
        }
    }

    pub fn expr_to_bb_value(&mut self, expr: &AstExpr) -> Value {
        match expr {
            AstExpr::Numeric { number, .. } => {
                if !number.contains('.') {
                    return Values::integer(number, 64);
                }
                panic!("floats currently unsupported for codegen sorry")
            }
            AstExpr::BinaryOperation { op, lhs, rhs, .. } => {
                let lhs_val = self.expr_to_bb_value(lhs);
                let rhs_val = self.expr_to_bb_value(rhs);
                match op {
                    BinaryOperationKind::Add => self.block.add(lhs_val, rhs_val),
                    BinaryOperationKind::Subtract => self.block.sub(lhs_val, rhs_val),
                    BinaryOperationKind::Multiply => self.block.mul(lhs_val, rhs_val),
                    BinaryOperationKind::Divide => self.block.sdiv(lhs_val, rhs_val),
                }
            }
            _ => {
                let loaded = self.expr_to_bb_ptr(expr);
                self.block.load(loaded.0, loaded.1)
            }
        }
    }

    pub fn expr_to_bb_ptr(&mut self, _expr: &AstExpr) -> (Type, Value) {
        panic!("ptr conversion is not supported yet")
    }
}
