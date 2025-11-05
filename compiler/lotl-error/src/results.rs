use crate::diagnostic::Diagnostic;

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
    pub fn bind<U, F: Fn(T) -> Results<U>>(self, f: F) -> Results<U> {
        let mut diag = self.diagnostics;
        let output = f(self.output);
        diag.extend(output.diagnostics);
        Results::new(output.output, diag)
    }

    /// Maps the data inside through the conversion provided.
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Results<U> {
        Results::new(f(self.output), self.diagnostics)
    }

    /// Forks the data to create a second pipeline of conversion.
    pub fn fork<U, F: Fn(&T) -> Results<U>>(self, f: F) -> Results<(T, U)> {
        let mut diag = self.diagnostics;
        let forked = f(&self.output);
        diag.extend(forked.diagnostics);
        Results::new((self.output, forked.output), diag)
    }

    /// Merges two results together.
    pub fn merge<B>(self, rhs: Results<B>) -> Results<(T, B)> {
        let mut diag = self.diagnostics;
        diag.extend(rhs.diagnostics);
        Results::new((self.output, rhs.output), diag)
    }
}