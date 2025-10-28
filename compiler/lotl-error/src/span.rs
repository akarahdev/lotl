use crate::file::SourceFile;

/// Represents a span of characters in a source file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    file: SourceFile,
    start: usize,
    end: usize,
}

impl Span {
    /// Creates a new span with the file and provided indices.
    pub fn new(file: SourceFile, start: usize, end: usize) -> Span {
        Self { file, start, end }
    }
}
