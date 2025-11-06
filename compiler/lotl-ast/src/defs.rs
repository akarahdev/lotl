use crate::ids::Tag;
use crate::stmt::AstStatement;
use crate::types::AstType;
use std::string::String;
use std::vec::Vec;
use uuid::Uuid;

/// Represents the ID of a top-level definition.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct AstDefinitionId(pub Uuid);

impl Tag for AstDefinitionId {
    type Input = String;

    fn make_new_from(input: &Self::Input) -> Self {
        // simple helper function, this is effectively a reimplementation
        // of java's String#hashCode function
        fn str_to_hash(s: &str) -> u128 {
            s.chars().fold(0, |hash, ch| 31 * hash + ch as u128)
        }
        
        AstDefinitionId(Uuid::from_u128(str_to_hash(input)))
    }
}

/// Represents a top-level definition in the AST.
#[derive(Debug, Clone, PartialEq)]
pub struct AstDefinition {
    /// The general name associated with the definition.
    pub name: String,
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
    /// Represents a namespace
    Namespace {
        /// The name of the namespace
        name: String,
        /// The members of the namespace
        members: Vec<AstDefinition>
    }
}

/// Represents an annotation on a top-level definition.
/// For example, `@value` is an annotation on a top-level structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct AstDefinitionAnnotation {
    /// The identifier of the annotation.
    pub name: String,
}
