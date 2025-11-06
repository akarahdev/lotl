use lotl_error::diagnostic::Diagnostic;
use lotl_token::{TokenKind, TokenStream, TokenTree};
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use lotl_ast::defs::AstDefinition;
use lotl_ast::expr::AstExpr;
use lotl_ast::graph::IdGraph;

pub struct Parser {
    pub(crate) vec: Arc<Vec<TokenTree>>,
    pub(crate) errors: RefCell<Vec<Diagnostic>>,
    pub(crate) index: Cell<usize>,

    pub(crate) definitions: IdGraph<AstDefinition>,
    pub(crate) exprs: IdGraph<AstExpr>
}

impl Parser {
    pub fn new(stream: TokenStream) -> Self {
        Parser {
            vec: stream.into_inner(),
            errors: RefCell::new(Vec::new()),
            index: Cell::new(0),

            definitions: IdGraph::new(),
            exprs: IdGraph::new()
        }
    }

    pub fn peek(&self) -> &TokenTree {
        self.vec
            .get(self.index.get())
            .unwrap_or_else(|| self.vec.last().unwrap())
    }

    pub fn next(&self) -> &TokenTree {
        if self.peek().kind == TokenKind::EndOfStream {
            return self.peek();
        }
        self.index.set(self.index.get() + 1);
        self.vec
            .get(self.index.get() - 1)
            .unwrap_or_else(|| self.vec.last().unwrap())
    }
}

#[macro_export]
/// Expects a token of a certain pattern, and fails with the error if it's not present
macro_rules! expect_kind {
    (
        $parser:expr,
        $tok:expr,
        $kind:pat,
        $expected:expr
    ) => {
        let $kind = $tok.kind else {
            $parser.push_err(Diagnostic::new(
                ExpectedKindFoundKind {
                    expected: $expected,
                    found: $tok.kind.clone(),
                },
                $tok.location.clone(),
            ));
            return None;
        };
    };
}
