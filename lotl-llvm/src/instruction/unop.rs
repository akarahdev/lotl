use crate::instruction::{BasicBlock, Instruction};
use crate::types::Type;
use crate::value::{LocalIdentifier, Value};
use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::String;

struct UnaryOp {
    returns_in: LocalIdentifier,
    operator: &'static str,
    ty: Box<dyn Type>,
    value: Box<dyn Value>,
}

impl IRComponent for UnaryOp {
    fn append_to_string(&self, string: &mut String) {
        self.returns_in.append_to_string(string);
        string.push_str(" = ");
        string.push_str(self.operator);
        string.push(' ');
        self.ty.append_to_string(string);
        string.push(' ');
        self.value.append_to_string(string);
    }
}
impl Instruction for UnaryOp {}

impl BasicBlock {
    pub fn fneg<T: Type + 'static, L: Value + 'static>(
        &mut self,
        ty: T,
        value: L,
    ) -> impl Value + 'static {
        let id = self.create_local_register();
        self.instructions.push(Box::new(UnaryOp {
            returns_in: id.clone(),
            operator: "fneg",
            ty: Box::new(ty),
            value: Box::new(value),
        }));
        id
    }
}

#[cfg(test)]
mod tests {}
