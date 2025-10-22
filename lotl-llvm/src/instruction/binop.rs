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

    pub fn mul(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: "mul",
            lhs,
            rhs,
        }));
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::module::{FunctionBody, GlobalFunction};
    use crate::types::Types;
    use crate::value::Values;
    use crate::IRComponent;
    use deranged::RangedU32;

    #[test]
    fn build_adding_function() {
        let body = FunctionBody::new(|block| {
            let summed = block.add(
                Values::integer("10", RangedU32::new(32).unwrap()).unwrap(),
                Values::integer("20", RangedU32::new(32).unwrap()).unwrap(),
            );
            block.ret(summed);
        });
        let f = GlobalFunction::new("main", Types::integer(RangedU32::new(32).unwrap())).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { entry: %r0 = add i32 10, 20 ret i32 %r0 }"
        );
    }

    #[test]
    fn build_multiply_function() {
        let body = FunctionBody::new(|block| {
            let product = block.mul(
                Values::integer("10", RangedU32::new(32).unwrap()).unwrap(),
                Values::integer("20", RangedU32::new(32).unwrap()).unwrap(),
            );
            block.ret(product);
        });
        let f = GlobalFunction::new("main", Types::integer(RangedU32::new(32).unwrap())).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { entry: %r0 = mul i32 10, 20 ret i32 %r0 }"
        );
    }
}
