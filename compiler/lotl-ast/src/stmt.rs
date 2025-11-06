use crate::expr::AstExpr;
use crate::ids::{Tag, Tagged};
use crate::types::AstType;
use uuid::Uuid;

/// Represents the ID of a statement in the AST.
#[derive(Debug, Clone, PartialEq)]
pub struct StatementId(pub Uuid);

impl Tag for StatementId {
    type Input = ();

    fn make_new_from(_input: &Self::Input) -> Self {
        StatementId(Uuid::new_v4())
    }
}

/// Represents a statement in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum AstStatement {
    /// A statement with branching conditions
    If {
        /// The condition to follow
        cond: AstExpr,
        /// Code to run if true
        if_true: Vec<AstStatement>,
        /// Code to run if false
        otherwise: Vec<AstStatement>,
        /// ID of the statement
        id: StatementId
    },
    /// A statement of storing data in a pointer
    Storage {
        /// The pointer to store into
        ptr: AstExpr,
        /// An optional type hint, if the variable is new
        type_hint: Option<AstType>,
        /// The value to write into the pointer
        value: AstExpr,
        /// ID of the statement
        id: StatementId,
    },
    /// Evaluates the expression and immediately drops the result.
    Drop {
        /// The expression to evaluate
        expr: AstExpr,
        /// ID of the statement
        id: StatementId,
    },
    /// Returns the value from the function
    Returns {
        /// The expression to return
        expr: AstExpr,
        /// ID of the statement
        id: StatementId
    }
}

impl Tagged<StatementId> for AstStatement {
    fn id(&self) -> &StatementId {
        match self {
            AstStatement::Storage { id, .. } => id,
            AstStatement::Drop { id, .. } => id,
            AstStatement::Returns { id, .. } => id,
            AstStatement::If { id, .. } => id
        }
    }
}
