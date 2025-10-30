use crate::expr::AstExpr;
use crate::types::AstType;

/// Represents a statement in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum AstStatement {
    /// A statement of storing data in a pointer
    Storage {
        /// The pointer to store into
        ptr: AstExpr,
        /// An optional type hint, if the variable is new
        type_hint: Option<AstType>,
        /// The value to write into the pointer
        value: AstExpr,
    },
    /// Evaluates the expression and immediately drops the result.
    Drop {
        /// The expression to evaluate
        expr: AstExpr,
    },
}
