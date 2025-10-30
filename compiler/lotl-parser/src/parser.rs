use lotl_error::diagnostic::Diagnostic;
use lotl_token::{TokenStream, TokenTree};
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

    pub fn peek(&self) -> Option<&TokenTree> {
        self.vec.get(self.index.get())
    }

    pub fn next(&self) -> Option<&TokenTree> {
        self.index.set(self.index.get() + 1);
        self.vec.get(self.index.get() - 1)
    }

    pub fn push_err(&self, diagnostic: Diagnostic) {
        self.errors.borrow_mut().push(diagnostic);
    }
    
    pub fn get_errs(&self) -> Vec<Diagnostic> {
        self.errors.borrow().clone()
    }
}
