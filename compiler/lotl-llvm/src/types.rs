use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use deranged::RangedU32;

/// Represents an LLVM IR Type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Type {
    /// Represents the LLVM integer type, with the number of bits specified.
    #[non_exhaustive]
    Integer(u32),
    /// Represents the LLVM array type, with the size and element type specified.
    #[non_exhaustive]
    Array(u32, Box<Type>),
    /// Represents a pointer into memory.
    #[non_exhaustive]
    Ptr,
    /// Represents the empty type with no size and value.
    #[non_exhaustive]
    Void,
    /// Represents a function that can be called.
    #[non_exhaustive]
    Function(Box<Type>, Vec<Box<Type>>),
}

impl IRComponent for Type {
    fn append_to_string(&self, string: &mut String) {
        match self {
            Type::Integer(width) => {
                string.push('i');
                string.push_str(&width.to_string());
            }
            Type::Array(size, subtype) => {
                string.push('[');
                string.push(' ');
                string.push_str(&size.to_string());
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
                    &parameters
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

/// A structure with implementations to generate type instances.
pub struct Types;

impl Types {
    /// Generates a new integer type, with a maximum width of (2^22 - 1)
    pub fn integer(width: RangedU32<0, 8388607>) -> Type {
        Type::Integer(width.get())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Type, Types};
    use crate::IRComponent;
    use alloc::boxed::Box;
    use deranged::RangedU32;

    #[test]
    pub fn test_integers() {
        let int = Types::integer(RangedU32::new(32).unwrap());
        assert_eq!(int.emit(), "i32");
    }

    #[test]
    pub fn test_arrays() {
        let int = Type::Array(4, Box::new(Types::integer(RangedU32::new(32).unwrap())));
        assert_eq!(int.emit(), "[ 4 x i32 ]");
    }
}
