use lotl_ast::expr::{AstExpr, BinaryOperationKind};
use lotl_ast::stmt::AstStatement;
use lotl_llvm_api::instruction::BasicBlock;
use lotl_llvm_api::types::Type;
use lotl_llvm_api::value::{Value, Values};

pub fn stmts_to_bb(stmts: &[AstStatement], bb: &mut BasicBlock) {
    for stmt in stmts {
        stmt_to_bb(stmt, bb);
    }
}

pub fn stmt_to_bb(stmt: &AstStatement, bb: &mut BasicBlock) {
    match stmt {
        AstStatement::Storage { ptr, value, .. } => {
            let target = expr_to_bb_ptr(ptr, bb);
            let val = expr_to_bb_value(value, bb);
            bb.store(target.1, val);
        }
        AstStatement::Drop { expr, .. } => {
            expr_to_bb_value(expr, bb);
        }
        AstStatement::Returns { expr, .. } => {
            let value = expr_to_bb_value(expr, bb);
            bb.ret(value);
        }
    }
}

pub fn expr_to_bb_value(expr: &AstExpr, bb: &mut BasicBlock) -> Value {
    match expr {
        AstExpr::Numeric { number, id, .. } => {
            if !number.contains('.') {
                return Values::integer(number, 64);
            }
            panic!("floats currently unsupported for codegen sorry")
        }
        AstExpr::BinaryOperation {
            op, lhs, rhs, id, ..
        } => {
            match op {
                BinaryOperationKind::Add => {
                    let lhs = expr_to_bb_value(lhs, bb);
                    let rhs = expr_to_bb_value(rhs, bb);
                    bb.add(lhs, rhs)
                }
                _ => panic!("unsupported op {op:?}")
            }
        }
        _ => {
            let loaded = expr_to_bb_ptr(expr, bb);
            bb.load(loaded.0, loaded.1)
        }
    }
}

pub fn expr_to_bb_ptr(expr: &AstExpr, bb: &mut BasicBlock) -> (Type, Value) {
    match expr {
        _ => panic!("unsupported converting to ptr or expr {expr:?}"),
    }
}
