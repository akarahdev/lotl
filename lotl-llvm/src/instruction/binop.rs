use crate::instruction::{BasicBlock, Instruction};
use crate::types::Type;
use crate::value::{LocalIdentifier, Value};
use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::String;

pub struct BinOp {
    returns_in: LocalIdentifier,
    operator: &'static str,
    ty: Box<dyn Type>,
    lhs: Box<dyn Value>,
    rhs: Box<dyn Value>,
}

impl IRComponent for BinOp {
    fn append_to_string(&self, string: &mut String) {
        self.returns_in.append_to_string(string);
        string.push_str(" = ");
        string.push_str(self.operator);
        string.push(' ');
        self.ty.append_to_string(string);
        string.push(' ');
        self.lhs.append_to_string(string);
        string.push_str(", ");
        self.rhs.append_to_string(string);
    }
}
impl Instruction for BinOp {}

impl BasicBlock {
    pub fn add<T: Type + 'static, L: Value + 'static, R: Value + 'static>(
        &mut self,
        ty: T,
        lhs: L,
        rhs: R,
    ) -> impl Value + 'static {
        let id = self.create_local_register();
        self.instructions.push(Box::new(BinOp {
            returns_in: id.clone(),
            operator: "add",
            ty: Box::new(ty),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }));
        id
    }
}

#[cfg(test)]
mod tests {
    use crate::module::{FunctionBody, GlobalFunction};
    use crate::types::Types;
    use crate::value::Values;
    use crate::{fn_ty, IRComponent};

    #[test]
    fn build_adding_function() {
        let body = FunctionBody::new(|block| {
            let summed = block.add(
                Types::integer(32),
                Values::integer("10").unwrap(),
                Values::integer("20").unwrap(),
            );
            block.ret(Types::integer(32), summed);
        });
        let f = GlobalFunction::new("main", fn_ty!(Types::integer(32))).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { entry: %r0 = add i32 10, 20 ret i32 %r0 }"
        );
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
        let f = GlobalFunction::new("main", fn_ty!(Types::integer(32))).body(body);
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
        let f = GlobalFunction::new("main", fn_ty!(Types::integer(32))).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { entry: br label %bb0 bb0: ret i32 120 }"
        );
    }
}
