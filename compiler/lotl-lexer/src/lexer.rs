use crate::err::{InvalidCharacter, UnexpectedEOFWhileFinding};
use lotl_error::Results;
use lotl_error::diagnostic::Diagnostic;
use lotl_error::file::SourceFile;
use lotl_error::span::Span;
use lotl_token::{TokenKind, TokenStream, TokenTree};

pub(crate) struct Lexer {
    file: SourceFile,
    diagnostics: Vec<Diagnostic>,
    index: usize,
    tracked_index: usize,
}

impl Lexer {
    pub fn new(file: SourceFile) -> Self {
        Self {
            file,
            diagnostics: Vec::new(),
            index: 0,
            tracked_index: 0,
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
                    self.diagnostics.push(Diagnostic::new(
                        UnexpectedEOFWhileFinding(terminating),
                        self.create_span(),
                    ))
                }
                vec.push(TokenTree::new(TokenKind::EndOfStream, self.create_span()));
                return TokenStream::new(vec);
            }
            if self.peek() == terminating {
                self.next();

                vec.push(TokenTree::new(TokenKind::EndOfStream, self.create_span()));
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
        self.tracked_index = self.index;
        if self.peek().is_ascii_alphabetic() {
            let mut str = String::new();
            while self.peek().is_ascii_alphabetic()
                || self.peek() == '_'
                || self.peek().is_ascii_digit()
            {
                str.push(self.next());
            }
            return match str.as_str() {
                "func" => Some(TokenTree::new(TokenKind::FuncKeyword, self.create_span())),
                "if" => Some(TokenTree::new(TokenKind::IfKeyword, self.create_span())),
                "else" => Some(TokenTree::new(TokenKind::ElseKeyword, self.create_span())),
                "let" => Some(TokenTree::new(TokenKind::LetKeyword, self.create_span())),
                "for" => Some(TokenTree::new(TokenKind::ForKeyword, self.create_span())),
                "while" => Some(TokenTree::new(TokenKind::WhileKeyword, self.create_span())),
                "return" => Some(TokenTree::new(TokenKind::ReturnKeyword, self.create_span())),
                _ => Some(TokenTree::new(TokenKind::Ident(str), self.create_span())),
            };
        }
        if self.peek().is_ascii_digit() {
            let mut str = String::new();
            while self.peek().is_ascii_digit() {
                str.push(self.next());
            }
            return Some(TokenTree::new(TokenKind::Numeric(str), self.create_span()));
        }
        match self.next() {
            ',' => Some(TokenTree::new(TokenKind::Comma, self.create_span())),
            ':' => Some(TokenTree::new(TokenKind::Colon, self.create_span())),
            ';' => Some(TokenTree::new(TokenKind::Semicolon, self.create_span())),

            '=' => Some(TokenTree::new(TokenKind::Equal, self.create_span())),
            '>' => Some(TokenTree::new(TokenKind::GreaterThan, self.create_span())),
            '<' => Some(TokenTree::new(TokenKind::LessThan, self.create_span())),

            '+' => Some(TokenTree::new(TokenKind::Plus, self.create_span())),
            '-' => {
                if self.peek() == '>' {
                    self.next();
                    Some(TokenTree::new(TokenKind::Arrow, self.create_span()))
                } else {
                    Some(TokenTree::new(TokenKind::Minus, self.create_span()))
                }
            }
            '*' => Some(TokenTree::new(TokenKind::Star, self.create_span())),
            '/' => Some(TokenTree::new(TokenKind::Slash, self.create_span())),
            '%' => Some(TokenTree::new(TokenKind::Percent, self.create_span())),
            '^' => Some(TokenTree::new(TokenKind::Caret, self.create_span())),

            '&' => Some(TokenTree::new(TokenKind::Ampersand, self.create_span())),
            '|' => Some(TokenTree::new(TokenKind::VerticalBar, self.create_span())),

            '!' => Some(TokenTree::new(
                TokenKind::ExclamationMark,
                self.create_span(),
            )),
            '?' => Some(TokenTree::new(TokenKind::QuestionMark, self.create_span())),

            '#' => Some(TokenTree::new(TokenKind::Hash, self.create_span())),
            '$' => Some(TokenTree::new(TokenKind::Dollar, self.create_span())),
            '.' => Some(TokenTree::new(TokenKind::Dot, self.create_span())),
            '@' => Some(TokenTree::new(TokenKind::At, self.create_span())),

            '{' => Some(TokenTree::new(
                TokenKind::Braces(self.lex_group('}')),
                self.create_span(),
            )),
            '(' => Some(TokenTree::new(
                TokenKind::Parenthesis(self.lex_group(')')),
                self.create_span(),
            )),
            '[' => Some(TokenTree::new(
                TokenKind::Brackets(self.lex_group(']')),
                self.create_span(),
            )),

            '\0' => None,
            ch => {
                self.diagnostics
                    .push(Diagnostic::new(InvalidCharacter(ch), self.create_span()));
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

    pub fn create_span(&self) -> Span {
        Span::new(self.file.clone(), self.tracked_index, self.index)
    }
}
