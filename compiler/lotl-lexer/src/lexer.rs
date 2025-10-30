use lotl_error::diagnostic::{Diagnostic, DiagnosticLevel};
use lotl_error::file::SourceFile;
use lotl_error::span::Span;
use lotl_error::Results;
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
        Results::new(self.lex_group('\0'), self.diagnostics)
    }

    pub fn lex_group(&mut self, terminating: char) -> TokenStream {
        let mut vec = Vec::new();
        loop {
            self.skip_whitespace();
            if self.index >= self.file.contents.len() {
                if terminating != '\0' {
                    self.diagnostics.push(Diagnostic::new_dynamic(
                        format!(
                            "Unexpected end of file while trying to find end of '{terminating:#?}'"
                        ),
                        DiagnosticLevel::Error,
                        self.single_char_span(),
                    ))
                }
                vec.push(TokenTree::new(
                    TokenKind::EndOfStream,
                    self.single_char_span(),
                ));
                return TokenStream::new(vec);
            }
            if self.peek() == terminating {
                self.next();

                vec.push(TokenTree::new(
                    TokenKind::EndOfStream,
                    self.single_char_span(),
                ));
                return TokenStream::new(vec);
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
            while self.peek().is_ascii_alphabetic()
                || self.peek() == '_'
                || self.peek().is_ascii_digit()
            {
                str.push(self.next());
            }
            return match str.as_str() {
                "func" => Some(TokenTree::new(
                    TokenKind::FuncKeyword,
                    self.single_char_span(),
                )),
                "if" => Some(TokenTree::new(
                    TokenKind::IfKeyword,
                    self.single_char_span(),
                )),
                "else" => Some(TokenTree::new(
                    TokenKind::ElseKeyword,
                    self.single_char_span(),
                )),
                "let" => Some(TokenTree::new(
                    TokenKind::LetKeyword,
                    self.single_char_span(),
                )),
                "for" => Some(TokenTree::new(
                    TokenKind::ForKeyword,
                    self.single_char_span(),
                )),
                "while" => Some(TokenTree::new(
                    TokenKind::WhileKeyword,
                    self.single_char_span(),
                )),
                "return" => Some(TokenTree::new(
                    TokenKind::ReturnKeyword,
                    self.single_char_span(),
                )),
                _ => Some(TokenTree::new(
                    TokenKind::Ident(str),
                    self.single_char_span(),
                )),
            };
        }
        if self.peek().is_ascii_digit() {
            let mut str = String::new();
            while self.peek().is_ascii_digit() {
                str.push(self.next());
            }
            return Some(TokenTree::new(
                TokenKind::Numeric(str),
                self.single_char_span(),
            ));
        }
        match self.next() {
            ',' => Some(TokenTree::new(TokenKind::Comma, self.single_char_span())),
            ':' => Some(TokenTree::new(TokenKind::Colon, self.single_char_span())),
            ';' => Some(TokenTree::new(
                TokenKind::Semicolon,
                self.single_char_span(),
            )),

            '=' => Some(TokenTree::new(TokenKind::Equal, self.single_char_span())),
            '>' => Some(TokenTree::new(
                TokenKind::GreaterThan,
                self.single_char_span(),
            )),
            '<' => Some(TokenTree::new(TokenKind::LessThan, self.single_char_span())),

            '+' => Some(TokenTree::new(TokenKind::Plus, self.single_char_span())),
            '-' => {
                if self.peek() == '>' {
                    self.next();
                    Some(TokenTree::new(TokenKind::Arrow, self.single_char_span()))
                } else {
                    Some(TokenTree::new(TokenKind::Minus, self.single_char_span()))
                }
            }
            '*' => Some(TokenTree::new(TokenKind::Star, self.single_char_span())),
            '/' => Some(TokenTree::new(TokenKind::Slash, self.single_char_span())),
            '%' => Some(TokenTree::new(TokenKind::Percent, self.single_char_span())),
            '^' => Some(TokenTree::new(TokenKind::Caret, self.single_char_span())),

            '&' => Some(TokenTree::new(
                TokenKind::Ampersand,
                self.single_char_span(),
            )),
            '|' => Some(TokenTree::new(
                TokenKind::VerticalBar,
                self.single_char_span(),
            )),

            '!' => Some(TokenTree::new(
                TokenKind::ExclamationMark,
                self.single_char_span(),
            )),
            '?' => Some(TokenTree::new(
                TokenKind::QuestionMark,
                self.single_char_span(),
            )),

            '#' => Some(TokenTree::new(TokenKind::Hash, self.single_char_span())),
            '$' => Some(TokenTree::new(TokenKind::Dollar, self.single_char_span())),
            '.' => Some(TokenTree::new(TokenKind::Dot, self.single_char_span())),
            '@' => Some(TokenTree::new(TokenKind::At, self.single_char_span())),

            '{' => Some(TokenTree::new(
                TokenKind::Braces(self.lex_group('}')),
                self.single_char_span(),
            )),
            '(' => Some(TokenTree::new(
                TokenKind::Parenthesis(self.lex_group(')')),
                self.single_char_span(),
            )),
            '[' => Some(TokenTree::new(
                TokenKind::Brackets(self.lex_group(']')),
                self.single_char_span(),
            )),

            '\0' => None,
            _ => {
                self.diagnostics.push(Diagnostic::new_static(
                    "Invalid character",
                    DiagnosticLevel::Error,
                    self.single_char_span(),
                ));
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

    pub fn single_char_span(&self) -> Span {
        Span::new(self.file.clone(), self.index, self.index)
    }
}
