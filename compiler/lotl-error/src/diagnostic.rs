use crate::span::Span;

/// Represents a single error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Diagnostic {
    /// Defines the diagnostic level
    pub level: DiagnosticLevel,
    /// The primary error
    pub main: (String, Span),
    /// Extra help of what to provide
    pub help: Option<(String, Span)>,
    /// An extra note about the error
    pub note: Option<(String, Span)>,
}

impl Diagnostic {
    /// Creates a new error instance.
    pub fn new<E: DiagnosticError>(error: E, span: Span) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            main: (error.message(), span),
            help: None,
            note: None,
        }
    }

    /// Sets the level of this diagnostic.
    pub fn level(mut self, level: DiagnosticLevel) -> Self {
        self.level = level;
        self
    }

    /// Sets the help information of this diagnostic.
    pub fn help(mut self, help: (String, Span)) -> Self {
        self.help = Some(help);
        self
    }

    /// Sets the note information of this diagnostic.
    pub fn note(mut self, note: (String, Span)) -> Self {
        self.note = Some(note);
        self
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

/// Represents a valid error for a diagnostic.
pub trait DiagnosticError {
    /// The message of this error.
    fn message(self) -> String;
}