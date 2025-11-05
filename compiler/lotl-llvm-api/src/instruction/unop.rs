use crate::IRComponent;
use crate::instruction::{BasicBlock, Instruction};
use crate::value::Value;
use std::boxed::Box;
use std::string::String;

struct UnaryOp {
    returns_in: String,
    operator: &'static str,
    value: Value,
}

impl IRComponent for UnaryOp {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.returns_in);
        string.push_str(" = ");
        string.push_str(self.operator);
        string.push(' ');
        self.value.append_to_string(string);
    }
}
impl Instruction for UnaryOp {}

impl BasicBlock {
    /// Negates the provided floating point.
    pub fn fneg(&mut self, value: Value) -> Value {
        let (name, out) = self.create_local_register(value.ty().clone());
        self.instructions.push(Box::new(UnaryOp {
            returns_in: name,
            operator: "fneg",
            value,
        }));
        out
    }
}

#[cfg(test)]
mod tests {}
