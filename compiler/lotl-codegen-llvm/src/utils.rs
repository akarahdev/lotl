use lotl_ast::types::AstType;
use lotl_llvm_api::types::{Type, Types};

pub fn ty_to_llvm(ty: &AstType) -> Type {
    match ty {
        AstType::Int32 => Types::integer(32),
        AstType::Int64 => Types::integer(64),
        AstType::Float32 => todo!(),
        AstType::Float64 => todo!(),
        AstType::Void => Types::void(),
        AstType::TypeVar(_) => todo!(),
        AstType::Unresolved(_) => todo!(),
    }
}
