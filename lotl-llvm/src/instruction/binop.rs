use crate::instruction::{BasicBlock, Instruction};
use crate::value::Value;
use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::String;

pub struct BinOp {
    returns_in: String,
    operator: &'static str,
    lhs: Value,
    rhs: Value,
}

impl IRComponent for BinOp {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.returns_in);
        string.push_str(" = ");
        string.push_str(self.operator);
        string.push(' ');
        self.lhs.ty().append_to_string(string);
        string.push(' ');
        self.lhs.append_to_string(string);
        string.push_str(", ");
        self.rhs.append_to_string(string);
    }
}
impl Instruction for BinOp {}

impl BasicBlock {
    pub fn add(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: "add",
            lhs,
            rhs,
        }));
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::module::{FunctionBody, GlobalFunction};
    use crate::types::Type;
    use crate::value::Value;
    use crate::IRComponent;
    use alloc::string::ToString;

    #[test]
    fn build_adding_function() {
        let body = FunctionBody::new(|block| {
            let summed = block.add(
                Value::Integer("10".to_string(), Type::Integer(32)),
                Value::Integer("20".to_string(), Type::Integer(32)),
            );
            block.ret(summed);
        });
        let f = GlobalFunction::new("main", Type::Integer(32)).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { entry: %r0 = add i32 10, 20 ret i32 %r0 }"
        );
    }

    #[test]
    fn build_cond_branching_function() {
        let body = FunctionBody::new(|block| {
            block.br_if(
                Value::Integer("1".to_string(), Type::Integer(1)),
                |true_label| {
                    true_label.ret(Value::Integer("120".to_string(), Type::Integer(32)));
                },
                |false_label| {
                    false_label.ret(Value::Integer("240".to_string(), Type::Integer(32)));
                },
            );
        });
        let f = GlobalFunction::new("main", Type::Integer(32)).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { entry: br i1 1, label %bb0, label %bb1 bb0: ret i32 120 bb1: ret i32 240 }"
        );
    }

    #[test]
    fn build_static_branching_function() {
        let body = FunctionBody::new(|block| {
            block.br(|true_label| {
                true_label.ret(Value::Integer("120".to_string(), Type::Integer(32)));
            });
        });
        let f = GlobalFunction::new("main", Type::Integer(32)).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { entry: br label %bb0 bb0: ret i32 120 }"
        );
    }
}
