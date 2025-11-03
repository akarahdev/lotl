use crate::file::SourceFile;
use std::fmt::Debug;

/// Represents a span of characters in a source file.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    file: SourceFile,
    start: usize,
    end: usize,
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?} @ {:?}..{:?}]", self.file, self.start, self.end)
    }
}

impl Span {
    /// Creates a new span with the file and provided indices.
    pub fn new(file: SourceFile, start: usize, end: usize) -> Span {
        Self { file, start, end }
    }
}
