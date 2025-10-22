use crate::instruction::{BasicBlock, Instruction};
use crate::types::{Type, Types};
use crate::value::{LocalIdentifier, Value};
use crate::IRComponent;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;

struct Return {
    ty: Box<dyn Type>,
    value: Option<Box<dyn Value>>,
}

impl IRComponent for Return {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("ret ");
        self.ty.append_to_string(string);
        self.value.iter().for_each(|value| {
            string.push(' ');
            value.append_to_string(string)
        });
    }
}
impl Instruction for Return {}

struct BranchCond {
    cond: (Box<dyn Type>, Box<dyn Value>),
    true_label: LocalIdentifier,
    false_label: LocalIdentifier,
}

impl IRComponent for BranchCond {
    fn append_to_string(&self, string: &mut String) {
        string.push_str(
            format!(
                "br {} {}, label {}, label {}",
                self.cond.0.emit(),
                self.cond.1.emit(),
                self.true_label.emit(),
                self.false_label.emit()
            )
            .as_str(),
        );
    }
}
impl Instruction for BranchCond {}

struct BranchConst {
    true_label: LocalIdentifier,
}

impl IRComponent for BranchConst {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("br label ");
        self.true_label.append_to_string(string);
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
    pub fn ret_void(&mut self) {
        self.instructions.push(Box::new(Return {
            ty: Box::new(Types::void()),
            value: None,
        }));
    }

    pub fn ret<T: Type + 'static, V: Value + 'static>(&mut self, ty: T, value: V) {
        self.instructions.push(Box::new(Return {
            ty: Box::new(ty),
            value: Some(Box::new(value)),
        }));
    }

    pub fn br<F: FnOnce(&mut BasicBlock)>(&mut self, label: F) {
        let br = Box::new(BranchConst {
            true_label: self.create_child(label),
        });
        self.instructions.push(br);
    }

    pub fn br_if<
        Ct: Type + 'static,
        Cv: Value + 'static,
        F1: FnOnce(&mut BasicBlock),
        F2: FnOnce(&mut BasicBlock),
    >(
        &mut self,
        ty: Ct,
        value: Cv,
        true_label: F1,
        false_label: F2,
    ) {
        let br = Box::new(BranchCond {
            cond: (Box::new(ty), Box::new(value)),
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
    use crate::{fn_ty, IRComponent};

    #[test]
    fn build_returning_function() {
        let body = FunctionBody::new(|block| {
            block.ret(Types::integer(32), Values::integer("120").unwrap());
        });
        let f = GlobalFunction::new(Values::global("main"), fn_ty!(Types::integer(32))).body(body);
        assert_eq!(f.emit(), "define i32 @main() { entry: ret i32 120 }");
    }

    #[test]
    fn build_cond_branching_function() {
        let body = FunctionBody::new(|block| {
            block.br_if(
                Types::integer(1),
                Values::integer("1").unwrap(),
                |true_label| {
                    true_label.ret(Types::integer(32), Values::integer("120").unwrap());
                },
                |false_label| {
                    false_label.ret(Types::integer(32), Values::integer("240").unwrap());
                },
            );
        });
        let f = GlobalFunction::new(Values::global("main"), fn_ty!(Types::integer(32))).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { entry: br i1 1, label %bb0, label %bb1 bb0: ret i32 120 bb1: ret i32 240 }"
        );
    }

    #[test]
    fn build_static_branching_function() {
        let body = FunctionBody::new(|block| {
            block.br(|true_label| {
                true_label.ret(Types::integer(32), Values::integer("120").unwrap());
            });
        });
        let f = GlobalFunction::new(Values::global("main"), fn_ty!(Types::integer(32))).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { entry: br label %bb0 bb0: ret i32 120 }"
        );
    }
}
