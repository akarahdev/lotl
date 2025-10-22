use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Represents an LLVM IR Type.
#[derive(Clone, Debug)]
pub enum Type {
    /// no im docing later
    Integer(u32),
    /// no im docing later
    Array(u32, Box<Type>),
    /// no im docing later
    Ptr,
    /// no im docing later
    Void,
    /// no im docing later
    Function(Box<Type>, Vec<Box<Type>>),
}

impl IRComponent for Type {
    fn append_to_string(&self, string: &mut String) {
        match self {
            Type::Integer(width) => {
                string.push('i');
                string.push_str(&*width.to_string());
            }
            Type::Array(size, subtype) => {
                string.push('[');
                string.push(' ');
                string.push_str(&*size.to_string());
                string.push_str(" x ");
                subtype.append_to_string(string);
                string.push(' ');
                string.push(']');
            }
            Type::Ptr => {
                string.push_str("ptr");
            }
            Type::Void => {
                string.push_str("void");
            }
            Type::Function(return_type, parameters) => {
                return_type.append_to_string(string);
                string.push('(');
                string.push_str(
                    &*parameters
                        .iter()
                        .map(|x| x.emit())
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                string.push(')');
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Type;
    use crate::IRComponent;
    use alloc::boxed::Box;

    #[test]
    pub fn test_integers() {
        let int = Type::Integer(32);
        assert_eq!(int.emit(), "i32");
    }

    #[test]
    pub fn test_arrays() {
        let int = Type::Array(4, Box::new(Type::Integer(32)));
        assert_eq!(int.emit(), "[ 4 x i32 ]");
    }
}
