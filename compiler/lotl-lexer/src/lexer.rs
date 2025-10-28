use lotl_error::file::SourceFile;
use lotl_error::span::Span;
use lotl_error::{Diagnostic, Results};
use lotl_token::{TokenKind, TokenStream, TokenTree};

pub(crate) struct Lexer {
    file: SourceFile,
    diagnostics: Vec<Diagnostic>,
    index: usize,
}

impl Lexer {
    pub fn new(file: SourceFile) -> Self {
        Self {
            file,
            diagnostics: Vec::new(),
            index: 0,
        }
    }

    pub fn lex_repeatedly(mut self) -> Results<TokenStream> {
        let mut vec = Vec::new();
        loop {
            self.skip_whitespace();
            if self.index >= self.file.contents.len() {
                return Results::new(TokenStream::new(vec), self.diagnostics);
            }
            if let Some(tok) = self.lex_once() {
                vec.push(tok);
            }
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            self.next();
        }
    }

    pub fn lex_once(&mut self) -> Option<TokenTree> {
        if self.peek().is_ascii_alphabetic() {
            let mut str = String::new();
            while self.peek().is_ascii_alphabetic() {
                str.push(self.next());
            }
            return match str.as_str() {
                "func" => Some(TokenTree::new(TokenKind::FuncKeyword, self.span())),
                "if" => Some(TokenTree::new(TokenKind::IfKeyword, self.span())),
                "else" => Some(TokenTree::new(TokenKind::ElseKeyword, self.span())),
                "let" => Some(TokenTree::new(TokenKind::LetKeyword, self.span())),
                "for" => Some(TokenTree::new(TokenKind::ForKeyword, self.span())),
                "while" => Some(TokenTree::new(TokenKind::WhileKeyword, self.span())),
                "return" => Some(TokenTree::new(TokenKind::ReturnKeyword, self.span())),
                _ => Some(TokenTree::new(TokenKind::Ident(str), self.span())),
            };
        }
        if self.peek().is_ascii_digit() {
            let mut str = String::new();
            while self.peek().is_ascii_digit() {
                str.push(self.next());
            }
            return Some(TokenTree::new(TokenKind::Numeric(str), self.span()));
        }
        match self.next() {
            ',' => Some(TokenTree::new(TokenKind::Comma, self.span())),
            ':' => Some(TokenTree::new(TokenKind::Colon, self.span())),
            ';' => Some(TokenTree::new(TokenKind::Semicolon, self.span())),

            '=' => Some(TokenTree::new(TokenKind::Equal, self.span())),
            '>' => Some(TokenTree::new(TokenKind::GreaterThan, self.span())),
            '<' => Some(TokenTree::new(TokenKind::LessThan, self.span())),

            '+' => Some(TokenTree::new(TokenKind::Plus, self.span())),
            '-' => Some(TokenTree::new(TokenKind::Minus, self.span())),
            '*' => Some(TokenTree::new(TokenKind::Star, self.span())),
            '/' => Some(TokenTree::new(TokenKind::Slash, self.span())),
            '%' => Some(TokenTree::new(TokenKind::Percent, self.span())),
            '^' => Some(TokenTree::new(TokenKind::Caret, self.span())),

            '&' => Some(TokenTree::new(TokenKind::Ampersand, self.span())),
            '|' => Some(TokenTree::new(TokenKind::VerticalBar, self.span())),

            '!' => Some(TokenTree::new(TokenKind::ExclamationMark, self.span())),
            '?' => Some(TokenTree::new(TokenKind::QuestionMark, self.span())),

            '#' => Some(TokenTree::new(TokenKind::Hash, self.span())),
            '$' => Some(TokenTree::new(TokenKind::Dollar, self.span())),
            '.' => Some(TokenTree::new(TokenKind::Dot, self.span())),
            '@' => Some(TokenTree::new(TokenKind::At, self.span())),

            '\0' => None,
            _ => {
                self.diagnostics
                    .push(Diagnostic::new_static("Invalid character", self.span()));
                None
            }
        }
    }

    pub fn peek(&self) -> char {
        self.file.contents.chars().nth(self.index).unwrap_or('\0')
    }

    pub fn next(&mut self) -> char {
        let ch = self.peek();
        self.index += 1;
        ch
    }

    pub fn span(&self) -> Span {
        Span::new(self.file.clone(), self.index, self.index)
    }
}
