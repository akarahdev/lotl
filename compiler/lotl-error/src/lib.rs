#![deny(missing_docs)]
//! This crate is responsible for Lotl's error handling, and also contains utilities for rendering.

use crate::span::Span;
use std::borrow::Cow;

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

/// Represents a single error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Diagnostic {
    /// The message the error contains.
    pub message: Cow<'static, str>,
    /// The span of the error with source file.
    pub span: Span,
}

impl Diagnostic {
    /// Creates a new error instance from a static string.
    pub fn new_static(message: &'static str, span: Span) -> Self {
        Self {
            message: Cow::Borrowed(message),
            span,
        }
    }

    /// Creates a new error instance from a dynamic string.
    pub fn new_dynamic(message: String, span: Span) -> Self {
        Self {
            message: Cow::Owned(message),
            span,
        }
    }
}
