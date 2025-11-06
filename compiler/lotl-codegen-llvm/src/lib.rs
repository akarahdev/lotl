#![deny(missing_docs)]
//! The crate responsible from converting an AST and type info to LLVM IR.

mod headers;
mod stmts;
mod utils;

use crate::headers::ast_to_header;
use lotl_ast::defs::AstDefinition;
use lotl_llvm_api::module::Module;
use lotl_typechk::context::TyContext;

/// Takes in the AST and type info and transforms it into an LLVM module.
pub fn codegen(code: (Vec<AstDefinition>, TyContext)) -> Module {
    let mut module = Module::new();
    for ast in code.0 {
        ast_to_header(&ast, &code.1, &mut module);
    }
    module
}

#[cfg(test)]
mod tests {
    use crate::codegen;
    use lotl_error::file::SourceFile;
    use lotl_lexer::lex;
    use lotl_llvm_api::IRComponent;
    use lotl_parser::parse;
    use lotl_typechk::infer_program;

    #[test]
    pub fn simple_return() {
        let source = SourceFile::new("example.lotl", "func start() -> i64 { return 10 + 20; }");
        let ast = lex(source)
            .bind(parse)
            .fork(infer_program)
            .peek(|x| {
                eprintln!("{:#?}", x);
            })
            .map(codegen);
        assert_eq!(ast.diagnostics.len(), 0);
    }

    #[test]
    pub fn branching() {
        let source = SourceFile::new(
            "example.lotl",
            "func start() -> i64 { \
             if 10 {\
                return 20;
             };
             return 40;\
         }",
        );
        let ast = lex(source)
            .bind(parse)
            .fork(infer_program)
            .peek(|x| {
                eprintln!("{:#?}", x);
            })
            .map(codegen);
        eprintln!("{:#?}", ast.diagnostics);
        eprintln!("{}", ast.output.emit());
        assert_eq!(ast.diagnostics.len(), 0);
    }
}
