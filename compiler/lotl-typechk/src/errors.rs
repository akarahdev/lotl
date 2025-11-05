use lotl_ast::types::AstType;
use lotl_error::diagnostic::DiagnosticError;

pub struct TypeMismatch<'a> {
    pub expected: &'a [AstType],
    pub found: &'a AstType
}

impl DiagnosticError for TypeMismatch<'_> {
    fn message(self) -> String {
        format!("expected one of {:?}, found {:?}", self.expected, self.found)
    }
}

pub struct VariableNotFound {
    pub name: String,
}

impl DiagnosticError for VariableNotFound {
    fn message(self) -> String {
        format!("variable {} not found in this scope", self.name)
    }
}