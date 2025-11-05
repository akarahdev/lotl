#![deny(missing_docs)]

//! Contains code for type inference and typechecking.

use crate::context::TyContext;
use crate::gatherer::{TypeGatherer, TypedStack};
use lotl_ast::defs::{AstDefinition, AstDefinitionKind};
use lotl_error::Results;
use std::ops::Deref;

/// Infers types for a given program.
pub fn infer_program(headers: &impl Deref<Target = [AstDefinition]>) -> Results<TyContext> {
    infer_program_with_ctx(headers, TyContext::new())
}

/// Infers types for a given program, with context provided from elsewhere.
pub fn infer_program_with_ctx(
    headers: &impl Deref<Target = [AstDefinition]>,
    mut ctx: TyContext,
) -> Results<TyContext> {
    for header in headers.deref() {
        infer_header_type(&mut ctx, header);
    }
    for header in headers.deref() {
        infer_header_data(&mut ctx, header);
    }
    let diag = ctx.diagnostics.clone();
    Results::new(ctx, diag)
}

/// Performs type inference on the provided header,
/// and modifies the provided type context.
pub fn infer_header_type(ctx: &mut TyContext, header: &AstDefinition) {
    let mut gatherer = TypeGatherer::new(ctx);
    gatherer.infer_header_type(header);
}

/// Performs type inference on the provided header's children,
/// and modifies the provided type context.
pub fn infer_header_data(ctx: &mut TyContext, header: &AstDefinition) {
    let mut gatherer = TypeGatherer::new(ctx);
    let mut stack = TypedStack::new();
    match &header.kind {
        AstDefinitionKind::Function { statements, .. } => {
            if let Some(stmts_vec) = statements {
                gatherer.infer_stmts(&mut stack, stmts_vec);
            }
        }
    }
}

/// Holds the data for types of AST entities
pub mod context;
mod errors;
pub(crate) mod gatherer;

#[cfg(test)]
mod tests {
    use crate::infer_program;
    use lotl_error::file::SourceFile;
    use lotl_lexer::lex;
    use lotl_parser::parse;

    #[test]
    fn binop_function() {
        let source = SourceFile::new("example.lotl", "func main() -> i32 { 10 + 20; }");
        let ast = lex(source).bind(parse).fork(infer_program);
        eprintln!("{ast:#?}");
        assert_eq!(ast.diagnostics.len(), 0);
    }
}
