use crate::types::AstType;
use alloc::string::String;
use alloc::vec::Vec;

/// Represents the ID of a top-level definition.
pub struct AstDefinitionId(pub u128);

/// Represents a top-level definition in the AST.
pub struct AstDefinition {
    /// The kind of definition this is.
    pub kind: AstDefinitionKind,
    /// The annotations applied to this definition.
    pub annotations: Vec<AstDefinitionAnnotation>,
    /// The ID of this definition.
    pub id: AstDefinitionId,
}

/// Represents the type of top level definition.
pub enum AstDefinitionKind {
    /// Represents a function definition.
    Function {
        /// The name of the function.
        name: String,
        /// The parameters that the function accepts.
        parameters: Vec<AstType>,
        /// The return type of the function.
        returns: AstType,
    },
}

/// Represents an annotation on a top-level definition.
/// For example, `@value` is an annotation on a top-level structure definition.
pub struct AstDefinitionAnnotation {
    /// The identifier of the annotation.
    pub name: String,
}
