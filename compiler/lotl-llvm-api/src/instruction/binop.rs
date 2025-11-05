use crate::instruction::{BasicBlockHandle, Instruction};
use crate::value::Value;
use crate::IRComponent;
use std::boxed::Box;
use std::string::String;

pub struct BinOp {
    returns_in: String,
    operator: BinaryOperator,
    lhs: Value,
    rhs: Value,
}

pub enum BinaryOperator {
    IntegerAdd,
    IntegerSub,
    IntegerMul,
    IntegerSignedDiv,
    IntegerUnsignedDiv,
    FloatAdd,
    FloatSub,
    FloatMul,
    FloatDiv,
}

impl IRComponent for BinaryOperator {
    fn append_to_string(&self, string: &mut String) {
        match self {
            BinaryOperator::IntegerAdd => string.push_str("add"),
            BinaryOperator::IntegerSub => string.push_str("sub"),
            BinaryOperator::IntegerMul => string.push_str("mul"),
            BinaryOperator::IntegerSignedDiv => string.push_str("sdiv"),
            BinaryOperator::IntegerUnsignedDiv => string.push_str("udiv"),
            BinaryOperator::FloatAdd => string.push_str("fadd"),
            BinaryOperator::FloatSub => string.push_str("fsub"),
            BinaryOperator::FloatMul => string.push_str("fmul"),
            BinaryOperator::FloatDiv => string.push_str("fdiv"),
        }
    }
}

impl IRComponent for BinOp {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.returns_in);
        string.push_str(" = ");
        self.operator.append_to_string(string);
        string.push(' ');
        self.lhs.append_to_string(string);
        string.push_str(", ");
        self.rhs.append_to_string_untyped(string);
    }
}
impl Instruction for BinOp {}

impl BasicBlockHandle<'_> {
    pub fn add(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: BinaryOperator::IntegerAdd,
            lhs,
            rhs,
        }));
        value
    }
    pub fn sub(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: BinaryOperator::IntegerSub,
            lhs,
            rhs,
        }));
        value
    }

    pub fn mul(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: BinaryOperator::IntegerMul,
            lhs,
            rhs,
        }));
        value
    }

    pub fn sdiv(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: BinaryOperator::IntegerSignedDiv,
            lhs,
            rhs,
        }));
        value
    }

    pub fn udiv(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: BinaryOperator::IntegerUnsignedDiv,
            lhs,
            rhs,
        }));
        value
    }

    pub fn fadd(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: BinaryOperator::FloatAdd,
            lhs,
            rhs,
        }));
        value
    }
    pub fn fsub(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: BinaryOperator::FloatSub,
            lhs,
            rhs,
        }));
        value
    }

    pub fn fmul(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: BinaryOperator::FloatMul,
            lhs,
            rhs,
        }));
        value
    }

    pub fn fdiv(&mut self, lhs: Value, rhs: Value) -> Value {
        let (name, value) = self.create_local_register(lhs.ty().clone());
        self.instructions.push(Box::new(BinOp {
            returns_in: name,
            operator: BinaryOperator::FloatDiv,
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

    #[test]
    fn build_adding_function() {
        let body = FunctionBody::new(|mut block| {
            let summed = block.add(Values::integer("10", 32), Values::integer("20", 32));
            block.ret(summed);
        });
        let f = GlobalFunction::new("main", Types::integer(32)).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { \
                entry: \
                    %r0 = add i32 10, 20 \
                    ret i32 %r0 \
            }"
        );
    }

    #[test]
    fn build_multiply_function() {
        let body = FunctionBody::new(|mut block| {
            let product = block.mul(
                Values::integer("10", 32),
                Values::integer("20", 32),
            );
            block.ret(product);
        });
        let f = GlobalFunction::new("main", Types::integer(32)).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { \
                entry: \
                    %r0 = mul i32 10, 20 \
                    ret i32 %r0 \
            }"
        );
    }

    #[test]
    fn build_dividing_function() {
        let body = FunctionBody::new(|mut block| {
            let product = block.sdiv(
                Values::integer("10", 32),
                Values::integer("20", 32),
            );
            let _product2 = block.udiv(
                Values::integer("10", 32),
                Values::integer("20", 32),
            );
            block.ret(product);
        });
        let f = GlobalFunction::new("main", Types::integer(32)).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { \
                entry: \
                    %r0 = sdiv i32 10, 20 \
                    %r1 = udiv i32 10, 20 \
                    ret i32 %r0 \
            }"
        );
    }
}
