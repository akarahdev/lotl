use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Represents an LLVM IR Type.
pub trait Type: IRComponent {}

/// A wrapper structure for constructing the different LLVM IR types.
/// All methods to create a valid LLVM IR type can be found here.
pub struct Types;

impl Types {
    /// The integer type is a type that represents an integer number. It requires a bit width
    /// to specify the size of the integer, but the space in memory will be rounded up to the
    /// nearest byte.
    ///
    /// Any bit width from 1 bit to 2^23(about 8 million) can be specified. Anything above that will
    /// result in an error.
    pub fn integer(width: u32) -> impl Type {
        if width >= 8388607 {
            panic!("Invalid width of integer type: {}", width);
        }
        Integer { width }
    }

    /// The pointer type is used to specify memory locations, to reference objects in memory.
    pub fn ptr() -> impl Type {
        Ptr
    }

    /// The array type is a type that arranges elements sequentially in memory.
    ///
    /// The array type requires a size (number of elements) and an underlying data type.
    ///
    /// There is no restriction on indexing beyond the end of the array implied by a static type
    /// (though there are restrictions on indexing beyond the bounds of an allocated object in
    /// some cases).
    pub fn array<T: Type + 'static>(size: u64, subtype: T) -> impl Type {
        Array {
            size,
            subtype: Box::new(subtype),
        }
    }

    /// The void type does not represent any value, and has no size.
    pub fn void() -> impl Type {
        Void
    }

    /// The function type can be thought of as a function signature.
    ///
    /// It consists of a return type and a list of formal parameter types.
    pub fn function<O: Type + 'static>(
        return_type: O,
        parameters: Vec<Box<dyn Type>>,
    ) -> FunctionPtr {
        FunctionPtr {
            return_type: Box::new(return_type),
            parameters,
        }
    }
}

/// This macro allows you to quickly construct a function pointer type.
#[macro_export]
macro_rules! fn_ty {
    ($output:expr) => {
        crate::types::Types::function($output, ::alloc::vec![])
    };
    ($output:expr, $($args:expr),*) => {
        crate::types::Types::function($output, ::alloc::vec![$($args,)*])
    };
}

struct Integer {
    pub width: u32,
}

impl IRComponent for Integer {
    fn append_to_string(&self, string: &mut String) {
        string.push('i');
        string.push_str(&self.width.to_string());
    }
}

impl Type for Integer {}

struct Ptr;

impl IRComponent for Ptr {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("ptr");
    }
}

impl Type for Ptr {}

struct Array {
    pub size: u64,
    pub subtype: Box<dyn Type>,
}

impl IRComponent for Array {
    fn append_to_string(&self, string: &mut String) {
        string.push('[');
        string.push(' ');
        string.push_str(&self.size.to_string());
        string.push_str(" x ");
        self.subtype.append_to_string(string);
        string.push(' ');
        string.push(']');
    }
}

impl Type for Array {}

struct Void;

impl IRComponent for Void {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("void");
    }
}

impl Type for Void {}

/// Represents a function pointer.
pub struct FunctionPtr {
    pub(crate) return_type: Box<dyn Type>,
    pub(crate) parameters: Vec<Box<dyn Type>>,
}

impl IRComponent for FunctionPtr {
    fn append_to_string(&self, string: &mut String) {
        self.return_type.append_to_string(string);
        string.push('(');
        string.push_str(
            &self
                .parameters
                .iter()
                .map(|x| x.emit())
                .collect::<Vec<_>>()
                .join(", "),
        );
        string.push(')');
    }
}

impl Type for FunctionPtr {}

#[cfg(test)]
mod tests {
    use crate::types::Types;
    use crate::IRComponent;

    #[test]
    pub fn test_integers() {
        let int = Types::integer(32);
        assert_eq!(int.emit(), "i32");
    }

    #[test]
    pub fn test_arrays() {
        let int = Types::array(15, Types::integer(32));
        assert_eq!(int.emit(), "[ 15 x i32 ]");
    }
}
