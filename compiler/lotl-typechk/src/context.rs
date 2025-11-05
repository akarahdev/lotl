use lotl_ast::defs::AstDefinitionId;
use lotl_ast::expr::ExprId;
use lotl_ast::types::AstType;
use lotl_error::diagnostic::Diagnostic;
use std::collections::HashMap;

/// Contains the context of types for given AST nodes
#[derive(Debug, Clone)]
pub struct TyContext {
    functions: HashMap<AstDefinitionId, FunctionSignature>,
    exprs: HashMap<ExprId, AstType>,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

/// Records the signature of a function.
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// The parameter types the function accepts
    pub parameters: Vec<AstType>,
    /// The return type of the function
    pub returns: AstType,
}

impl Default for TyContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TyContext {
    /// Creates a new empty type context
    pub fn new() -> Self {
        TyContext {
            functions: HashMap::new(),
            exprs: HashMap::new(),
            diagnostics: Vec::new(),
        }
    }

    /// Records the type of the given expression
    pub fn record_expr(&mut self, id: &ExprId, ty: AstType) {
        self.exprs.insert(id.clone(), ty);
    }

    /// Gets the type of the given expression
    pub fn type_of_expr(&self, id: &ExprId) -> Option<&AstType> {
        self.exprs.get(id)
    }

    /// Records the type of the given expression
    pub fn record_func(&mut self, id: &AstDefinitionId, ty: FunctionSignature) {
        self.functions.insert(id.clone(), ty);
    }

    /// Gets the type of the given expression
    pub fn type_of_func(&self, id: &AstDefinitionId) -> Option<&FunctionSignature> {
        self.functions.get(id)
    }
}
