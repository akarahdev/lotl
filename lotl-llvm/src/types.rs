use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub trait Type: IRComponent {}

pub struct Types;

impl Types {
    pub fn integer(width: u64) -> impl Type {
        Integer { width }
    }

    pub fn array<T: Type + 'static>(size: u64, subtype: T) -> impl Type {
        Array {
            size,
            subtype: Box::new(subtype),
        }
    }

    pub fn void() -> impl Type {
        Void
    }

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

#[macro_export]
macro_rules! fn_ty {
    ($output:expr) => {
        crate::types::Types::function($output, ::alloc::vec![])
    };
    ($output:expr, $($args:expr),*) => {
        crate::types::Types::function($output, ::alloc::vec![$($args,)*])
    };
}

pub struct Integer {
    pub width: u64,
}

impl IRComponent for Integer {
    fn append_to_string(&self, string: &mut String) {
        string.push('i');
        string.push_str(&self.width.to_string());
    }
}

impl Type for Integer {}

pub struct Array {
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

pub struct Void;

impl IRComponent for Void {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("void");
    }
}

impl Type for Void {}

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
