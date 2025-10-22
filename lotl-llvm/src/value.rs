use crate::types::{Type, Types};
use crate::IRComponent;
use alloc::string::String;
use deranged::RangedU32;

/// Represents a valid LLVM value.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Value {
    /// Represents a constant integer value. The value is of integer type.
    #[non_exhaustive]
    Integer(String, Type),
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
}

impl IRComponent for Value {
    fn append_to_string(&self, string: &mut String) {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Type;
    use crate::value::{Value, Values};
    use crate::IRComponent;
    use alloc::string::ToString;
    use deranged::RangedU32;

    #[test]
    pub fn test_local_idents() {
        let value = Value::LocalIdentifier("foo".to_string(), Type::Ptr);
        assert_eq!(value.emit(), "%foo");
    }
    #[test]
    pub fn test_global_idents() {
        let value = Value::GlobalIdentifier("foo".to_string(), Type::Ptr);
        assert_eq!(value.emit(), "@foo");
    }
    #[test]
    pub fn test_int_constants() {
        let value = Values::integer("1256", RangedU32::new(32).unwrap()).unwrap();
        assert_eq!(value.emit(), "1256");
    }
}
