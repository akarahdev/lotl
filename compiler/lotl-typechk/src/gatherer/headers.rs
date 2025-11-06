use crate::context::FunctionSignature;
use crate::gatherer::TypeGatherer;
use lotl_ast::defs::{AstDefinition, AstDefinitionKind};

impl<'a> TypeGatherer<'a> {
    pub fn infer_header_type(&mut self, header: &AstDefinition) {
        match &header.kind {
            AstDefinitionKind::Function {
                parameters,
                returns,
                ..
            } => self.ctx.record_func(
                &header.id,
                FunctionSignature {
                    parameters: parameters.clone(),
                    returns: returns.clone(),
                },
            ),
            AstDefinitionKind::Namespace { members, .. } => {
                for member in members {
                    self.infer_header_type(member);
                }
            }
        }
    }
}
