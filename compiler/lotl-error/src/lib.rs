#![deny(missing_docs)]
//! This crate is responsible for Lotl's error handling, and also contains utilities for rendering.

use crate::diagnostic::Diagnostic;

/// Contains information for diagnostics and errors.
pub mod diagnostic;
/// Contains the source file abstraction.
pub mod file;
/// Contains the Span structure.
pub mod span;

/// Represents a value and a possible series of errors.
/// If errors are present, the output is only partial.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Results<T> {
    /// Represents the output data, even if partial.
    pub output: T,
    /// Represents the error diagnostics.
    pub diagnostics: Vec<Diagnostic>,
}

impl<T> Results<T> {
    /// Creates a new set of errors.
    pub fn new(output: T, diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            output,
            diagnostics,
        }
    }

    /// Applies the result of this value to a function returning another result set.
    /// If errors are present, both series of errors are concatenated.
    pub fn and_then<U, F: Fn(T) -> Results<U>>(self, f: F) -> Results<U> {
        let mut diag = self.diagnostics;
        let output = f(self.output);
        diag.extend(output.diagnostics);
        Results::new(output.output, diag)
    }
}
