//! A simple crate for defining the tokens used in Lotl.
//!
//! This is mostly used by the Lexer (which creates the tokens), a Parser (which turns the tokens
//! into an AST). This crate also provides the SourceFile and Span types which are used for file
//! inputs and error handling.

#![deny(missing_docs)]

use std::sync::Arc;

/// Represents a source file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFile {
    name: Arc<String>,
    contents: Arc<String>,
}

impl SourceFile {
    /// Creates a new source file from a name and contents.
    pub fn new(name: String, contents: String) -> SourceFile {
        Self {
            name: Arc::new(name),
            contents: Arc::new(contents),
        }
    }
}

/// Represents a span of characters in a source file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    file: Arc<SourceFile>,
    start: usize,
    end: usize,
}

impl Span {
    /// Creates a new span with the file and provided indices.
    pub fn new(file: Arc<SourceFile>, start: usize, end: usize) -> Span {
        Self {
            file,
            start,
            end,
        }
    }
}

/// Represents a tree of tokens.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenTree {
    kind: TokenKind,
    location: Span
}

/// Represents the kind of token being stored.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    /// Represents the root of a file.
    Root(Vec<TokenTree>),

    /// Represents an identifier.
    Ident(String),
    /// Represents any number, floating-point or integer.
    Numeric(String),
    /// Represents a string literal.
    StringLiteral(String),

    /// Represents tokens wrapped in braces.
    Braces(Vec<TokenTree>),
    /// Represents tokens wrapped in brackets.
    Brackets(Vec<TokenTree>),
    /// Represents tokens wrapped inside parentheses.
    Parenthesis(Vec<TokenTree>),

    /// Represents a comment.
    Comment(String),

    /// Represents a comma: `,`
    Comma,
    /// Represents a colon: `:`
    Colon,
    /// Represents a semicolon: `;`
    Semicolon,

    /// Represents an equal sign: `=`
    Equal,
    /// Represents a greater than sign: `>`
    GreaterThan,
    /// Represents a less than sign: `<`
    LessThan,

    /// Represents a plus sign: `+`
    Plus,
    /// Represents a minus sign: `-`
    Minus,
    /// Represents a star sign: `*`
    Star,
    /// Represents a forward slash: `/`
    Slash,
    /// Represents a percent sign: `%`
    Percent,
    /// Represents a caret sign: `^`
    Caret,

    /// Represents an ampersand sign: `&`
    Ampersand,
    /// Represents a vertical bar: `|`
    VerticalBar,

    /// Represents a question mark: `?`
    QuestionMark,
    /// Represents an exclamation mark: `!`
    ExclamationMark,

    /// Represents a hash sign: `#`
    Hash,
    /// Represents a dollar sign: `$`
    Dollar,
    /// Represents a dot: `.`
    Dot,
    /// Represents an at sign: `@`
    At,
}