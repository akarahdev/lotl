use crate::stmts::CodegenContext;
use crate::utils::ty_to_llvm;
use lotl_ast::defs::{AstDefinition, AstDefinitionKind};
use lotl_llvm_api::module::{FunctionBody, GlobalFunction, Module};
use lotl_typechk::context::TyContext;

pub fn ast_to_header(ast: &AstDefinition, ctx: &TyContext, module: &mut Module) {
    match &ast.kind {
        AstDefinitionKind::Function {
            name,
            parameters,
            returns,
            statements,
            ..
        } => {
            let mut func = GlobalFunction::new(name, ty_to_llvm(returns));
            for param in parameters {
                func = func.with_parameter(ty_to_llvm(param));
            }
            if let Some(stmts) = statements {
                let fb = FunctionBody::new(|bb| {
                    let mut ctx = CodegenContext {
                        types: ctx,
                        block: bb,
                    };
                    ctx.stmts_to_bb(stmts);
                });
                func = func.body(fb);
            }
            module.functions.push(func);
        }
    }
}
