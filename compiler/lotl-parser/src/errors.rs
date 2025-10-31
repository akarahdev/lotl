use crate::parser::Parser;
use lotl_error::diagnostic::{Diagnostic, DiagnosticError};
use lotl_token::TokenKind;

impl Parser {
    pub fn push_err(&self, diagnostic: Diagnostic) {
        self.errors.borrow_mut().push(diagnostic);
    }

    pub fn get_errs(&self) -> Vec<Diagnostic> {
        self.errors.borrow().clone()
    }
}

pub struct ExpectedKindFoundKind<'a> {
    pub expected: &'a [TokenKind],
    pub found: TokenKind,
}

impl<'a> DiagnosticError for ExpectedKindFoundKind<'a> {
    fn message(self) -> String {
        format!(
            "Expected {}, but found {}",
            self.expected
                .iter()
                .map(|x| x.name())
                .collect::<Vec<_>>()
                .join(", or "),
            self.found.name()
        )
    }
}
