use lotl_error::span::Span;

/// Represents an expression in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum AstExpr {
    /// An identifier
    Identifier {
        /// The name of the identifier
        name: String,
        /// The span of the identifier
        span: Span,
    },
    /// Represents a numeric value
    Numeric {
        /// The value of the number
        number: String,
        /// The span of the numeric value
        span: Span,
    },
    /// Represents a binary operation of 2 expressions
    BinaryOperation {
        /// The binary operator to use
        op: BinaryOperationKind,
        /// The left-hand side of the operation
        lhs: Box<AstExpr>,
        /// The right-hand side of the operation
        rhs: Box<AstExpr>,
        /// The span of the operator
        op_span: Span,
    },
    /// Represents a unary operation applied to an expression
    UnaryOperation {
        /// The unary operation to use
        op: UnaryOperationKind,
        /// The expression to apply the operation to
        expr: Box<AstExpr>,
        /// The span of the operator
        op_span: Span,
    },
    /// Represents a function invocations
    Invocation {
        /// The function to invoke
        func: Box<AstExpr>,
        /// The parameters to invoke the function with
        parameters: Vec<AstExpr>,
    },
    /// Represents a field access
    FieldAccess {
        /// The object to access the field of
        obj: Box<AstExpr>,
        /// The field to access
        field: String,
    },
    /// Represents a namespace access
    NamespaceAccess {
        /// The namespace to access the field of
        obj: Box<AstExpr>,
        /// The path to access
        path: String,
    },
    /// Represents a subscript
    Subscript {
        /// The object to access the index of
        obj: Box<AstExpr>,
        /// The value to index
        index: Box<AstExpr>,
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
