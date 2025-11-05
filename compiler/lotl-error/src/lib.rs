#![deny(missing_docs)]
//! This crate is responsible for Lotl's error handling, and also contains utilities for rendering.

/// Contains information for diagnostics and errors.
pub mod diagnostic;
/// Contains the source file abstraction.
pub mod file;
/// Contains result structures for error handling and incomplete data.
pub mod results;
/// Contains the Span structure.
pub mod span;

