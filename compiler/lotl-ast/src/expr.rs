/// Represents an expression in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum AstExpr {
    /// An identifier
    Identifier {
        /// The name of the identifier
        name: String,
    },
    /// Represents a numeric value
    Numeric {
        /// The value of the number
        number: String,
    },
    /// Represents a binary operation of 2 expressions
    BinaryOperation {
        /// The binary operator to use
        op: BinaryOperationKind,
        /// The left-hand side of the operation
        lhs: Box<AstExpr>,
        /// The right-hand side of the operation
        rhs: Box<AstExpr>,
    },
    /// Represents a unary operation applied to an expression
    UnaryOperation {
        /// The unary operation to use
        op: UnaryOperationKind,
        /// The expression to apply the operation to
        expr: Box<AstExpr>,
    },
}

/// The possible kinds of binary operations
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperationKind {
    /// `+` operator
    Add,
    /// `-` operator
    Subtract,
    /// `*` operator
    Multiply,
    /// `/` operator
    Divide,
}

/// The possible kinds of unary operations
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperationKind {
    /// `-` operator
    Negate,
    /// `!` operator
    Not,
}
