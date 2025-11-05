use crate::instruction::{BasicBlock, Instruction};
use crate::types::Type;
use crate::value::Value;
use crate::IRComponent;
use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;

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

pub struct GetElementPtr {
    returns_in: String,
    ty: Type,
    base: Value,
    indices: Vec<Value>,
}

impl IRComponent for GetElementPtr {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.returns_in);
        string.push_str(" = getelementptr ");
        self.ty.append_to_string(string);
        string.push_str(", ");
        self.base.append_to_string(string);
        for value in &self.indices {
            string.push_str(", ");
            value.append_to_string(string);
        }
    }
}
impl Instruction for GetElementPtr {}

impl BasicBlock {
    /// Extracts a value out of the aggregate at the index.
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

    /// Inserts a value into the aggregate at the index.
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

    /// Gets a pointer to the element at the aggregate at the index
    pub fn getelementptr(&mut self, ty: Type, base: Value, indices: Vec<Value>) -> Value {
        let mut param_ty: Type = ty.clone();
        for index in &indices {
            match param_ty {
                Type::Array(_, param) => {
                    param_ty = *param.clone();
                }
                Type::Structure(params) => {
                    let Value::Integer(length, _) = index.clone() else {
                        panic!("getelementptr index for a structure must be an integer");
                    };
                    param_ty = params
                        .get(length.parse::<usize>().unwrap())
                        .unwrap()
                        .clone();
                }
                _ => panic!("getelementptr requires a structure or array type"),
            }
        }
        let (name, value) = self.create_local_register(Type::Ptr);
        self.instructions.push(Box::new(GetElementPtr {
            returns_in: name,
            ty,
            base,
            indices,
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
    use std::vec;

    #[test]
    fn build_extracting_function() {
        let body = FunctionBody::new(|block| {
            let summed = block.extractvalue(
                Values::structure(vec![Values::integer("10", 32), Values::integer("20", 64)]),
                0,
            );
            block.ret(summed);
        });
        let f = GlobalFunction::new("main", Types::integer(32)).body(body);
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
        let body = FunctionBody::new(|block| {
            let init_struct = Values::zeroinitializer(Types::structure(vec![
                Types::integer(32),
                Types::integer(64),
            ]));
            let inserted_0 = block.insertvalue(init_struct, Values::integer("10", 32), 0);
            let _inserted_1 = block.insertvalue(inserted_0, Values::integer("20", 64), 1);
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
