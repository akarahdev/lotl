use lotl_error::diagnostic::Diagnostic;
use lotl_token::{TokenKind, TokenStream, TokenTree};
use std::cell::{Cell, RefCell};
use std::sync::Arc;

pub struct Parser {
    pub(crate) vec: Arc<Vec<TokenTree>>,
    pub(crate) errors: RefCell<Vec<Diagnostic>>,
    pub(crate) index: Cell<usize>,
}

impl Parser {
    pub fn new(stream: TokenStream) -> Self {
        Parser {
            vec: stream.into_inner(),
            errors: RefCell::new(Vec::new()),
            index: Cell::new(0),
        }
    }

    pub fn peek(&self) -> &TokenTree {
        self.vec.get(self.index.get()).unwrap_or_else(|| {
            self.vec.last().unwrap()
        })
    }

    pub fn next(&self) -> &TokenTree {
        if self.peek().kind == TokenKind::EndOfStream {
            return self.peek();
        }
        self.index.set(self.index.get() + 1);
        self.vec.get(self.index.get() - 1).unwrap_or_else(|| {
            self.vec.last().unwrap()
        })
    }

    pub fn push_err(&self, diagnostic: Diagnostic) {
        self.errors.borrow_mut().push(diagnostic);
    }

    pub fn get_errs(&self) -> Vec<Diagnostic> {
        self.errors.borrow().clone()
    }
}

#[macro_export]
/// Expects a token of a certain pattern, and fails with the error if it's not present
macro_rules! expect_kind {
    (
        $parser:expr,
        $tok:expr,
        $kind:pat
    ) => {
        let $kind = $tok.kind else {
            $parser.push_err(Diagnostic::new_dynamic(
                format!("Expected {:?}, found {:?}", stringify!($pat), $tok.kind),
                DiagnosticLevel::Error,
                $tok.location.clone(),
            ));
            return None;
        };
    };
}
