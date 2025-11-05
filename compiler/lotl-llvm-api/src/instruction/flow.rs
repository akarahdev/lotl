use crate::instruction::{BasicBlock, Instruction};
use crate::types::Type;
use crate::value::Value;
use crate::IRComponent;
use std::boxed::Box;
use std::format;
use std::string::String;

struct Return {
    value: Option<Value>,
}

impl IRComponent for Return {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("ret ");
        let v = Type::Void;
        self.value
            .as_ref()
            .map(|x| x.ty())
            .unwrap_or_else(|| &v)
            .append_to_string(string);
        self.value.iter().for_each(|value| {
            string.push(' ');
            value.append_to_string_untyped(string)
        });
    }
}
impl Instruction for Return {}

struct BranchCond {
    cond: Value,
    true_label: String,
    false_label: String,
}

impl IRComponent for BranchCond {
    fn append_to_string(&self, string: &mut String) {
        string.push_str(
            format!(
                "br {}, label %{}, label %{}",
                self.cond.emit(),
                self.true_label,
                self.false_label
            )
            .as_str(),
        );
    }
}
impl Instruction for BranchCond {}

struct BranchConst {
    true_label: String,
}

impl IRComponent for BranchConst {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("br label %");
        string.push_str(&self.true_label);
    }
}
impl Instruction for BranchConst {}

struct Unreachable;
impl IRComponent for Unreachable {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("unreachable");
    }
}
impl Instruction for Unreachable {}

impl BasicBlock {
    /// Returns void.
    pub fn ret_void(&mut self) {
        self.instructions.push(Box::new(Return { value: None }));
    }

    /// Returns the given value.
    pub fn ret(&mut self, value: Value) {
        self.instructions
            .push(Box::new(Return { value: Some(value) }));
    }

    /// Marks the end of this block unreachable.
    pub fn unreachable(&mut self) {
        self.instructions.push(Box::new(Unreachable));
    }

    /// Branches to the label unconditionally.
    pub fn br<F: FnOnce(&mut BasicBlock)>(&mut self, label: F) {
        let br = Box::new(BranchConst {
            true_label: self.create_child(label),
        });
        self.instructions.push(br);
    }

    /// Branches to the basic block if true, otherwise goes to the false label.
    pub fn br_if<F1: FnOnce(&mut BasicBlock), F2: FnOnce(&mut BasicBlock)>(
        &mut self,
        value: Value,
        true_label: F1,
        false_label: F2,
    ) {
        let br = Box::new(BranchCond {
            cond: value,
            true_label: self.create_child(true_label),
            false_label: self.create_child(false_label),
        });
        self.instructions.push(br);
    }
}

#[cfg(test)]
mod tests {
    use crate::module::{FunctionBody, GlobalFunction};
    use crate::types::Types;
    use crate::value::Values;
    use crate::IRComponent;

    #[test]
    fn build_returning_function() {
        let body = FunctionBody::new(|block| {
            block.ret(Values::integer("120", 32));
        });
        let f = GlobalFunction::new("main", Types::integer(32)).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { \
                entry: ret i32 120 \
            }"
        );
    }

    #[test]
    fn build_unreachable_function() {
        let body = FunctionBody::new(|block| {
            block.unreachable();
        });
        let f = GlobalFunction::new("main", Types::integer(32)).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { \
                entry: \
                    unreachable \
            }"
        );
    }

    #[test]
    fn build_cond_branching_function() {
        let body = FunctionBody::new(|block| {
            block.br_if(
                Values::integer("1", 1),
                |true_label| {
                    true_label.ret(Values::integer("120", 32));
                },
                |false_label| {
                    false_label.ret(Values::integer("240", 32));
                },
            );
        });
        let f = GlobalFunction::new("main", Types::integer(32)).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { \
                entry: \
                    br i1 1, label %bb0, label %bb1 \
                bb0: \
                    ret i32 120 \
                bb1: \
                    ret i32 240 \
            }"
        );
    }

    #[test]
    fn build_static_branching_function() {
        let body = FunctionBody::new(|block| {
            block.br(|true_label| {
                true_label.ret(Values::integer("120", 32));
            });
        });
        let f = GlobalFunction::new("main", Types::integer(32)).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { \
                entry: \
                    br label %bb0 \
                bb0: \
                    ret i32 120 \
            }"
        );
    }
}
