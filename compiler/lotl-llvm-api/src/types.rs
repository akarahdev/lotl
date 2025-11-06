use crate::IRComponent;
use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;

/// Represents an LLVM IR Type.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Type {
    /// Represents the LLVM integer type, with the number of bits specified.
    #[non_exhaustive]
    Integer(u32),
    /// Represents the LLVM array type, with the size and element type specified.
    #[non_exhaustive]
    Array(u32, Box<Type>),
    /// Represents the LLVM structure type, with the element types specified.
    #[non_exhaustive]
    Structure(Vec<Type>),
    /// Represents a pointer into memory.
    #[non_exhaustive]
    Ptr,
    /// Represents the empty type with no size and value.
    #[non_exhaustive]
    Void,
    /// Represents a function that can be called.
    #[non_exhaustive]
    Function(Box<Type>, Vec<Box<Type>>),
    /// Represents a basic 16-bit floating point.
    Half,
    /// A 32-bit floating point.
    Float,
    /// A 64-bit floating point.
    Double,
    /// A 128-bit floating point.
    FP128,
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
            Type::Structure(parameters) => {
                string.push('{');
                string.push_str(
                    &parameters
                        .iter()
                        .map(|x| x.emit())
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                string.push('}');
            }
            Type::Half => string.push_str("half"),
            Type::Float => string.push_str("float"),
            Type::Double => string.push_str("double"),
            Type::FP128 => string.push_str("fp128"),
        }
    }
}

/// A structure with implementations to generate type instances.
pub struct Types;

impl Types {
    /// Generates a new integer type, with a maximum width of (2^22 - 1)
    pub fn integer(width: u32) -> Type {
        Type::Integer(width)
    }
    /// Generates a new array type, with the specified length and element type
    pub fn array(length: u32, subtype: Type) -> Type {
        Type::Array(length, Box::new(subtype))
    }

    /// Generates a new structure type, with the provided element types.
    pub fn structure(subtypes: Vec<Type>) -> Type {
        Type::Structure(subtypes)
    }

    /// Generates a new void type, with no size or value.
    pub fn void() -> Type {
        Type::Void
    }

    /// Generates a new 16-bit floating point.
    pub fn fp16() -> Type {
        Type::Half
    }

    /// Generates a new 32-bit floating point.
    pub fn fp32() -> Type {
        Type::Float
    }

    /// Generates a new 64-bit floating point.
    pub fn fp64() -> Type {
        Type::Double
    }

    /// Generates a new 128-bit floating point.
    pub fn fp128() -> Type {
        Type::FP128
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Type, Types};
    use crate::IRComponent;
    use std::vec;

    #[test]
    pub fn test_integers() {
        let int = Types::integer(32);
        assert_eq!(int.emit(), "i32");
    }

    #[test]
    pub fn test_arrays() {
        let int = Types::array(4, Types::integer(32));
        assert_eq!(int.emit(), "[ 4 x i32 ]");
    }
    #[test]
    pub fn test_structures() {
        let int = Type::Structure(vec![Type::Integer(32), Type::Integer(64)]);
        assert_eq!(int.emit(), "{i32, i64}");
    }
}
