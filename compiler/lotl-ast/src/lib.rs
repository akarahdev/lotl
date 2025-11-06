//! This is the crate holding Lotl's AST definitions.
//! The purpose is for a separate crate to hold all the definitions that other crates
//! can share the AST types across.
//! All queries involving manipulating or transforming the AST should be hold in separate crates.

#![deny(missing_docs)]

/// Defines the top-level definitions
pub mod defs;
/// Defines the expressions of a top-level definition
pub mod expr;
/// Defines the ID graph structure that holds nodes and their IDs.
pub mod graph;
/// Contains generic code for AST IDs across the codebase
pub mod ids;
/// Defines the type system of the AST
pub mod types;
