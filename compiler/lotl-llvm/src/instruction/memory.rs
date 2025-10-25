use crate::IRComponent;
use crate::instruction::{BasicBlockHandle, Instruction};
use crate::types::Type;
use crate::value::Value;
use std::boxed::Box;
use std::string::String;

pub struct StoreValue {
    value: Value,
    pointer: Value,
}

impl IRComponent for StoreValue {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("store ");
        self.value.append_to_string(string);
        string.push_str(", ");
        self.pointer.append_to_string(string);
    }
}
impl Instruction for StoreValue {}

pub struct LoadValue {
    returns_in: String,
    ty: Type,
    pointer: Value,
}

impl IRComponent for LoadValue {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.returns_in);
        string.push_str(" = load ");
        self.ty.append_to_string(string);
        string.push_str(", ");
        self.pointer.append_to_string(string);
    }
}
impl Instruction for LoadValue {}

pub struct Alloca {
    returns_in: String,
    ty: Type,
}

impl IRComponent for Alloca {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.returns_in);
        string.push_str(" = alloca ");
        self.ty.append_to_string(string);
    }
}
impl Instruction for Alloca {}

impl BasicBlockHandle<'_> {
    pub fn store(&mut self, value: Value, pointer: Value) {
        self.instructions
            .push(Box::new(StoreValue { value, pointer }));
    }

    pub fn load(&mut self, ty: Type, pointer: Value) -> Value {
        let (name, value) = self.create_local_register(ty.clone());
        self.instructions.push(Box::new(LoadValue {
            returns_in: name,
            ty,
            pointer,
        }));
        value
    }

    pub fn alloca(&mut self, ty: Type) -> Value {
        let (name, value) = self.create_local_register(Type::Ptr);
        self.instructions.push(Box::new(Alloca {
            returns_in: name,
            ty,
        }));
        value
    }

    // pub fn extractvalue(&mut self, structure: Value, index: usize) -> Value {
    //     match structure.ty() {
    //         Type::Structure(parameters) => {
    //             let (name, value) =
    //                 self.create_local_register(parameters.get(index).unwrap().clone());
    //             self.instructions.push(Box::new(ExtractValue {
    //                 returns_in: name,
    //                 structure,
    //                 index,
    //             }));
    //             value
    //         }
    //         Type::Array(length, element) => {
    //             if index > (*length as usize) {
    //                 panic!("extractvalue index out of bounds");
    //             }
    //             let (name, value) = self.create_local_register(*element.clone());
    //             self.instructions.push(Box::new(ExtractValue {
    //                 returns_in: name,
    //                 structure,
    //                 index,
    //             }));
    //             value
    //         }
    //         _ => panic!("extractvalue requires a structure or array type"),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::IRComponent;
    use crate::module::{FunctionBody, GlobalFunction};
    use crate::types::Types;
    use crate::value::Values;
    use deranged::RangedU32;
    use std::vec;

    #[test]
    fn build_allocating_function() {
        let body = FunctionBody::new(|mut block| {
            let stack_ptr = block.alloca(Types::integer(RangedU32::new(32).unwrap()));
            block.store(
                Values::integer("10", RangedU32::new(32).unwrap()).unwrap(),
                stack_ptr.clone(),
            );
            let loaded = block.load(Types::integer(RangedU32::new(32).unwrap()), stack_ptr);
            block.ret(loaded);
        });
        let f = GlobalFunction::new("main", Types::integer(RangedU32::new(32).unwrap())).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { \
                entry: \
                    %r0 = alloca i32 \
                    store i32 10, ptr %r0 \
                    %r1 = load i32, ptr %r0 \
                    ret i32 %r1 \
            }"
        );
    }

    #[test]
    fn build_getelementptr_function() {
        let body = FunctionBody::new(|mut block| {
            let struct_type = Types::structure(vec![
                Types::integer(RangedU32::new(32).unwrap()),
                Types::integer(RangedU32::new(32).unwrap()),
            ]);
            let struct_const = Values::zeroinitializer(struct_type.clone());
            let struct_const_1 = block.insertvalue(
                struct_const,
                Values::integer("0", RangedU32::new(32).unwrap()).unwrap(),
                0,
            );
            let struct_const_2 = block.insertvalue(
                struct_const_1,
                Values::integer("1", RangedU32::new(32).unwrap()).unwrap(),
                1,
            );
            let stack_ptr = block.alloca(struct_type.clone());

            block.store(struct_const_2, stack_ptr.clone());

            let second_element_ptr = block.getelementptr(
                struct_type.clone(),
                stack_ptr.clone(),
                vec![Values::integer("1", RangedU32::new(32).unwrap()).unwrap()],
            );
            let loaded = block.load(
                Types::integer(RangedU32::new(32).unwrap()),
                second_element_ptr,
            );
            block.ret(loaded);
        });
        let f = GlobalFunction::new("main", Types::integer(RangedU32::new(32).unwrap())).body(body);
        assert_eq!(
            f.emit(),
            "define i32 @main() { \
                entry: \
                    %r0 = insertvalue {i32, i32} zeroinitializer, i32 0, 0 \
                    %r1 = insertvalue {i32, i32} %r0, i32 1, 1 \
                    %r2 = alloca {i32, i32} \
                    store {i32, i32} %r1, ptr %r2 \
                    %r3 = getelementptr {i32, i32}, ptr %r2, i32 1 \
                    %r4 = load i32, ptr %r3 \
                    ret i32 %r4 \
            }"
        );
    }
}
