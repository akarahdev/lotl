use crate::ids::{Tag, Tagged};
use lotl_error::span::Span;
use uuid::Uuid;

/// Represents the ID of an AST expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprId(pub Uuid);

impl Tag for ExprId {
    type Input = ();

    fn make_new_from(_input: &Self::Input) -> Self {
        ExprId(Uuid::new_v4())
    }
}

/// Represents an expression in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum AstExpr {
    /// An identifier
    Identifier {
        /// The name of the identifier
        name: String,
        /// The span of the identifier
        span: Span,
        /// The ID of the expression
        id: ExprId,
    },
    /// Represents a numeric value
    Numeric {
        /// The value of the number
        number: String,
        /// The span of the numeric value
        span: Span,
        /// The ID of the expression
        id: ExprId,
    },
    /// Represents a binary operation of 2 expressions
    BinaryOperation {
        /// The binary operator to use
        op: BinaryOperationKind,
        /// The left-hand side of the operation
        lhs: ExprId,
        /// The right-hand side of the operation
        rhs: ExprId,
        /// The span of the operator
        op_span: Span,
        /// The ID of the expression
        id: ExprId,
    },
    /// Represents a unary operation applied to an expression
    UnaryOperation {
        /// The unary operation to use
        op: UnaryOperationKind,
        /// The expression to apply the operation to
        expr: ExprId,
        /// The span of the operator
        op_span: Span,
        /// The ID of the expression
        id: ExprId,
    },
    /// Represents a function invocations
    Invocation {
        /// The function to invoke
        obj: ExprId,
        /// The parameters to invoke the function with
        parameters: Vec<ExprId>,
        /// The ID of the expression
        id: ExprId,
    },
    /// Represents a field access
    FieldAccess {
        /// The object to access the field of
        obj: ExprId,
        /// The field to access
        field: String,
        /// The ID of the expression
        id: ExprId,
    },
    /// Represents a namespace access
    NamespaceAccess {
        /// The namespace to access the field of
        obj: ExprId,
        /// The path to access
        path: String,
        /// The ID of the expression
        id: ExprId,
    },
    /// Represents a subscript
    Subscript {
        /// The object to access the index of
        obj: ExprId,
        /// The value to index
        index: ExprId,
        /// The ID of the expression
        id: ExprId,
    },
}

impl Tagged for AstExpr {
    type TagType = ExprId;

    fn id(&self) -> &ExprId {
        match self {
            AstExpr::Identifier { id, .. } => id,
            AstExpr::Numeric { id, .. } => id,
            AstExpr::BinaryOperation { id, .. } => id,
            AstExpr::UnaryOperation { id, .. } => id,
            AstExpr::Invocation { id, .. } => id,
            AstExpr::FieldAccess { id, .. } => id,
            AstExpr::NamespaceAccess { id, .. } => id,
            AstExpr::Subscript { id, .. } => id,
        }
    }
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
