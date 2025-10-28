use std::fmt::Debug;
use std::sync::Arc;

/// Represents a source file.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFile {
    /// The name of the file.
    pub name: Arc<String>,
    /// The raw string contents of the file.
    pub contents: Arc<String>,
}

impl Debug for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
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
