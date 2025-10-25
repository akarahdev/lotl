use crate::IRComponent;
use crate::types::{Type, Types};
use deranged::RangedU32;
use std::string::String;
use std::vec::Vec;

/// Represents a valid LLVM value.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Value {
    /// Represents a constant integer value. The value is of integer type.
    #[non_exhaustive]
    Integer(String, Type),
    /// Represents an LLVM zero-initializer.
    #[non_exhaustive]
    ZeroInitializer(Type),
    /// Represents a LLVM constant structure.
    #[non_exhaustive]
    Structure(Vec<Value>, Type),
    /// Represents a global identifier. This is always of pointer type.
    #[non_exhaustive]
    GlobalIdentifier(String, Type),
    /// Represents a local identifier in a function.
    #[non_exhaustive]
    LocalIdentifier(String, Type),
}

impl Value {
    /// Returns the type associated with this value.
    pub fn ty(&self) -> &Type {
        match self {
            Value::Integer(_, ty) => ty,
            Value::GlobalIdentifier(_, ty) => ty,
            Value::LocalIdentifier(_, ty) => ty,
            Value::Structure(_, ty) => ty,
            Value::ZeroInitializer(ty) => ty,
        }
    }
}

/// A structure with implementations to generate values.
pub struct Values;

impl Values {
    /// Generates a new integer constant, with a maximum width of (2^22 - 1).
    pub fn integer(contents: &str, size: RangedU32<0, 8388607>) -> Option<Value> {
        for (idx, ch) in contents.chars().enumerate() {
            if !(ch.is_ascii_digit() || (ch == '-' && idx == 0)) {
                return None;
            }
        }
        Some(Value::Integer(
            contents.parse().unwrap(),
            Types::integer(size),
        ))
    }

    /// Creates a new constant structure value, with the provided values as elements
    pub fn structure(contents: Vec<Value>) -> Value {
        Value::Structure(
            contents.clone(),
            Types::structure(contents.iter().map(|x| x.ty().clone()).collect()),
        )
    }

    /// Creates a new zero-initialized value
    pub fn zeroinitializer(ty: Type) -> Value {
        Value::ZeroInitializer(ty)
    }
}

impl IRComponent for Value {
    fn append_to_string(&self, string: &mut String) {
        self.ty().append_to_string(string);
        string.push(' ');
        self.append_to_string_untyped(string);
    }
}

impl Value {
    pub(crate) fn append_to_string_untyped(&self, string: &mut String) {
        match self {
            Value::Integer(value, _) => {
                string.push_str(value);
            }
            Value::GlobalIdentifier(name, _) => {
                string.push('@');
                string.push_str(name);
            }
            Value::LocalIdentifier(name, _) => {
                string.push('%');
                string.push_str(name);
            }
            Value::Structure(elements, _) => {
                string.push('{');
                string.push_str(
                    elements
                        .iter()
                        .map(Value::emit)
                        .collect::<Vec<_>>()
                        .join(", ")
                        .as_str(),
                );
                string.push('}');
            }
            Value::ZeroInitializer(_) => string.push_str("zeroinitializer"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IRComponent;
    use crate::types::Type;
    use crate::value::{Value, Values};
    use deranged::RangedU32;
    use std::string::ToString;
    use std::vec;

    #[test]
    pub fn test_local_idents() {
        let value = Value::LocalIdentifier("foo".to_string(), Type::Ptr);
        assert_eq!(value.emit(), "ptr %foo");
    }
    #[test]
    pub fn test_global_idents() {
        let value = Value::GlobalIdentifier("foo".to_string(), Type::Ptr);
        assert_eq!(value.emit(), "ptr @foo");
    }
    #[test]
    pub fn test_int_constants() {
        let value = Values::integer("1256", RangedU32::new(32).unwrap()).unwrap();
        assert_eq!(value.emit(), "i32 1256");
    }
    #[test]
    pub fn test_structure_constants() {
        let value = Values::structure(vec![
            Values::integer("1256", RangedU32::new(32).unwrap()).unwrap(),
        ]);
        assert_eq!(value.emit(), "{i32} {i32 1256}");
    }
}
