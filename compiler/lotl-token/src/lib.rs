//! A simple crate for defining the tokens used in Lotl.
//!
//! This is mostly used by the Lexer (which creates the tokens), a Parser (which turns the tokens
//! into an AST). This crate also provides the SourceFile and Span types which are used for file
//! inputs and error handling.

#![deny(missing_docs)]

pub use lotl_error::span::Span;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::Arc;

/// Represents a stream of token trees. Can be called with `.iter()` to start iterating over it.
/// These can also be cheaply copied around.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TokenStream {
    inner: Arc<Vec<TokenTree>>,
}

impl Debug for TokenStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", &self.inner))
    }
}

impl Deref for TokenStream {
    type Target = Vec<TokenTree>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TokenStream {
    /// Creates a new token stream from an array of token trees.
    pub fn new(inner: Vec<TokenTree>) -> TokenStream {
        Self {
            inner: Arc::new(inner),
        }
    }

    /// Creates an empty token stream.
    pub fn empty() -> TokenStream {
        Self::new(vec![])
    }

    /// Gets the internal vector of a TokenStream
    pub fn into_inner(self) -> Arc<Vec<TokenTree>> {
        self.inner
    }
}

/// Represents a tree of tokens.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenTree {
    /// The type of token this is
    pub kind: TokenKind,
    /// The source file of this token
    pub location: Span,
}

impl TokenTree {
    /// Creates a new token tree.
    pub fn new(kind: TokenKind, location: Span) -> TokenTree {
        TokenTree { kind, location }
    }
}

/// Represents the kind of token being stored.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    /// Represents an identifier.
    Ident(String),
    /// Represents any number, floating-point or integer.
    Numeric(String),
    /// Represents a string literal.
    StringLiteral(String),

    /// Represents tokens wrapped in braces.
    Braces(TokenStream),
    /// Represents tokens wrapped in brackets.
    Brackets(TokenStream),
    /// Represents tokens wrapped inside parentheses.
    Parenthesis(TokenStream),

    /// Represents a comment.
    Comment(String),

    /// The `func` keyword
    FuncKeyword,
    /// The `if` keyword
    IfKeyword,
    /// The `else` keyword
    ElseKeyword,
    /// The `let` keyword
    LetKeyword,
    /// The `return` keyword
    ReturnKeyword,
    /// The `while` keyword
    WhileKeyword,
    /// The `for` keyword
    ForKeyword,

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

    /// Represents a right arrow: `->`
    Arrow,

    /// Represents the end of a file
    EndOfStream,
}

impl TokenKind {
    /// Returns the name of this token.
    pub fn name(&self) -> &'static str {
        match self {
            TokenKind::Ident(_) => "identifier",
            TokenKind::Numeric(_) => "number",
            TokenKind::StringLiteral(_) => "string",
            TokenKind::Braces(_) => "braces",
            TokenKind::Brackets(_) => "brackets",
            TokenKind::Parenthesis(_) => "parenthesis",
            TokenKind::Comment(_) => "comment",
            TokenKind::FuncKeyword => "func",
            TokenKind::IfKeyword => "if",
            TokenKind::ElseKeyword => "else",
            TokenKind::LetKeyword => "let",
            TokenKind::ReturnKeyword => "return",
            TokenKind::WhileKeyword => "while",
            TokenKind::ForKeyword => "for",
            TokenKind::Comma => ",",
            TokenKind::Colon => ":",
            TokenKind::Semicolon => ";",
            TokenKind::Equal => "=",
            TokenKind::GreaterThan => ">",
            TokenKind::LessThan => "<",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Caret => "^",
            TokenKind::Ampersand => "^",
            TokenKind::VerticalBar => "|",
            TokenKind::QuestionMark => "?",
            TokenKind::ExclamationMark => "!",
            TokenKind::Hash => "#",
            TokenKind::Dollar => "$",
            TokenKind::Dot => ".",
            TokenKind::At => "@",
            TokenKind::Arrow => "->",
            TokenKind::EndOfStream => "EOF",
        }
    }
}
