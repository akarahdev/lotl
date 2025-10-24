use crate::instruction::{BasicBlockHandle, Instruction};
use crate::types::Type;
use crate::value::Value;
use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::{String, ToString};

pub struct ExtractValue {
    returns_in: String,
    structure: Value,
    index: usize,
}

impl IRComponent for ExtractValue {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.returns_in);
        string.push_str(" = extractvalue ");

        self.structure.append_to_string(string);
        string.push_str(", ");
        string.push_str(&self.index.to_string());
    }
}
impl Instruction for ExtractValue {}

pub struct InsertValue {
    returns_in: String,
    structure: Value,
    insertion: Value,
    index: usize,
}

impl IRComponent for InsertValue {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.returns_in);
        string.push_str(" = insertvalue ");

        self.structure.append_to_string(string);
        string.push_str(", ");
        self.insertion.append_to_string(string);
        string.push_str(", ");
        string.push_str(&self.index.to_string());
    }
}
impl Instruction for InsertValue {}

impl BasicBlockHandle<'_> {
    pub fn extractvalue(&mut self, structure: Value, index: usize) -> Value {
        match structure.ty() {
            Type::Structure(parameters) => {
                let (name, value) =
                    self.create_local_register(parameters.get(index).unwrap().clone());
                self.instructions.push(Box::new(ExtractValue {
                    returns_in: name,
                    structure,
                    index,
                }));
                value
            }
            Type::Array(length, element) => {
                if index > (*length as usize) {
                    panic!("extractvalue index out of bounds");
                }
                let (name, value) = self.create_local_register(*element.clone());
                self.instructions.push(Box::new(ExtractValue {
                    returns_in: name,
                    structure,
                    index,
                }));
                value
            }
            _ => panic!("extractvalue requires a structure or array type"),
        }
    }

    pub fn insertvalue(&mut self, structure: Value, insertion: Value, index: usize) -> Value {
        match structure.ty() {
            Type::Structure(parameters) => {
                let (name, value) = self.create_local_register(structure.ty().clone());
                if parameters.get(index).unwrap().clone() != *insertion.ty() {
                    panic!(
                        "expected parameter type {:?}, found {:?}",
                        value.ty(),
                        insertion.ty()
                    );
                }
                self.instructions.push(Box::new(InsertValue {
                    returns_in: name,
                    structure,
                    insertion,
                    index,
                }));
                value
            }
            Type::Array(length, element) => {
                if index > (*length as usize) {
                    panic!("insertvalue index out of bounds");
                }
                let (name, value) = self.create_local_register(structure.ty().clone());
                if **element != *insertion.ty() {
                    panic!(
                        "expected parameter type {:?}, found {:?}",
                        value.ty(),
                        insertion.ty()
                    );
                }
                self.instructions.push(Box::new(InsertValue {
                    returns_in: name,
                    structure,
                    insertion,
                    index,
                }));
                value
            }
            _ => panic!("insertvalue requires a structure or array type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::module::{FunctionBody, GlobalFunction};
    use crate::types::Types;
    use crate::value::Values;
    use crate::IRComponent;
    use alloc::vec;
    use deranged::RangedU32;

    #[test]
    fn build_extracting_function() {
        let body = FunctionBody::new(|mut block| {
            let summed = block.extractvalue(
                Values::structure(vec![
                    Values::integer("10", RangedU32::new(32).unwrap()).unwrap(),
                    Values::integer("20", RangedU32::new(64).unwrap()).unwrap(),
                ]),
                0,
            );
            block.ret(summed);
        });
        let f = GlobalFunction::new("main", Types::integer(RangedU32::new(32).unwrap())).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { \
                entry: \
                    %r0 = extractvalue {i32, i64} {i32 10, i64 20}, 0 \
                    ret i32 %r0 \
            }"
        );
    }

    #[test]
    fn build_inserting_function() {
        let body = FunctionBody::new(|mut block| {
            let init_struct = Values::zeroinitializer(Types::structure(vec![
                Types::integer(RangedU32::new(32).unwrap()),
                Types::integer(RangedU32::new(64).unwrap()),
            ]));
            let inserted_0 = block.insertvalue(
                init_struct,
                Values::integer("10", RangedU32::new(32).unwrap()).unwrap(),
                0,
            );
            let _inserted_1 = block.insertvalue(
                inserted_0,
                Values::integer("20", RangedU32::new(64).unwrap()).unwrap(),
                1,
            );
            block.ret_void();
        });
        let f = GlobalFunction::new("main", Types::void()).body(body);
        assert_eq!(
            f.emit(),
            "define void @main() { \
                entry: \
                    %r0 = insertvalue {i32, i64} zeroinitializer, i32 10, 0 \
                    %r1 = insertvalue {i32, i64} %r0, i64 20, 1 \
                    ret void \
            }"
        );
    }
}
