use std::string::String;

/// Represents Lotl's type system in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum AstType {
    /// Represents a 32-bit integer.
    Int32,
    /// Represents a 64-bit integer.
    Int64,
    /// Represents a 32-bit floating point number.
    Float32,
    /// Represents a 64-bit floating point number.
    Float64,
    ///  an empty type.
    Void,
    /// Represents a generic type variable.
    TypeVar(String),
    /// Represents a type that is not yet resolved.
    Unresolved(String),
}
