use crate::span::Span;
use std::borrow::Cow;

/// Represents a single error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Diagnostic {
    /// The message the error contains.
    pub message: Cow<'static, str>,
    /// The span of the error with source file.
    pub span: Span,
    /// Defines the diagnostic level
    pub level: DiagnosticLevel
}

impl Diagnostic {
    /// Creates a new error instance from a static string.
    pub fn new_static(message: &'static str, level: DiagnosticLevel, span: Span) -> Self {
        Self {
            message: Cow::Borrowed(message),
            span,
            level,
        }
    }

    /// Creates a new error instance from a dynamic string.
    pub fn new_dynamic(message: String, level: DiagnosticLevel, span: Span) -> Self {
        Self {
            message: Cow::Owned(message),
            span,
            level,
        }
    }
}

/// The level of the diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticLevel {
    /// Indicates this diagnostic is a warning
    Warning,
    /// Indicates this diagnostic is an error, failing compilation
    Error,
    /// Indicates this diagnostic is purely informational
    Info,
}
