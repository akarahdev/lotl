use crate::instruction::{Instruction, SharedBasicBlock};
use crate::types::Type;
use crate::value::Value;
use crate::IRComponent;
use std::boxed::Box;
use std::string::String;

struct Trunc {
    returns_in: String,
    value: Value,
    target: Type,
}

impl IRComponent for Trunc {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.returns_in);
        string.push_str(" = ");
        string.push_str("trunc ");
        self.value.append_to_string(string);
        string.push_str(" to ");
        self.target.append_to_string(string);
    }
}
impl Instruction for Trunc {}

impl SharedBasicBlock {
    /// Truncates the provided value.
    pub fn trunc(&mut self, value: Value, target: Type) -> Value {
        let (name, out) = self.create_local_register(target.clone());
        self.push_instruction(Box::new(Trunc {
            returns_in: name,
            value,
            target,
        }));
        out
    }
}

#[cfg(test)]
mod tests {}
