use std::sync::Arc;

/// Represents a source file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFile {
    /// The name of the file.
    pub name: Arc<String>,
    /// The raw string contents of the file.
    pub contents: Arc<String>,
}

impl SourceFile {
    /// Creates a new source file from a name and contents.
    pub fn new(name: &str, contents: &str) -> SourceFile {
        Self {
            name: Arc::new(String::from(name)),
            contents: Arc::new(String::from(contents)),
        }
    }
}
