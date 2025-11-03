use crate::stmt::AstStatement;
use crate::types::AstType;
use std::string::String;
use std::vec::Vec;
use uuid::Uuid;

/// Represents the ID of a top-level definition.
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct AstDefinitionId(pub Uuid);

/// Represents a top-level definition in the AST.
#[derive(Debug, Clone, PartialEq)]
pub struct AstDefinition {
    /// The kind of definition this is.
    pub kind: AstDefinitionKind,
    /// The annotations applied to this definition.
    pub annotations: Vec<AstDefinitionAnnotation>,
    /// The ID of this definition.
    pub id: AstDefinitionId,
}

/// Represents the type of top level definition.
#[derive(Debug, Clone, PartialEq)]
pub enum AstDefinitionKind {
    /// Represents a function definition.
    Function {
        /// The name of the function.
        name: String,
        /// The parameters that the function accepts.
        parameters: Vec<AstType>,
        /// The generic names of the function.
        generics: Vec<String>,
        /// The return type of the function.
        returns: AstType,
        /// The statements of the function.
        statements: Option<Vec<AstStatement>>,
    },
}

/// Represents an annotation on a top-level definition.
/// For example, `@value` is an annotation on a top-level structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct AstDefinitionAnnotation {
    /// The identifier of the annotation.
    pub name: String,
}
