use crate::types::Type;
use crate::IRComponent;
use alloc::string::String;

/// Represents a valid LLVM value.
#[derive(Clone, Debug)]
pub enum Value {
    /// Represents a constant integer value. The value is of integer type.
    Integer(String, Type),
    /// Represents a global identifier. This is always of pointer type.
    GlobalIdentifier(String, Type),
    /// Represents a local identifier in a function.
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

impl IRComponent for Value {
    fn append_to_string(&self, string: &mut String) {
        match self {
            Value::Integer(value, ty) => {
                string.push_str(value);
            }
            Value::GlobalIdentifier(name, ty) => {
                string.push('@');
                string.push_str(name);
            }
            Value::LocalIdentifier(name, ty) => {
                string.push('%');
                string.push_str(name);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Type;
    use crate::value::Value;
    use crate::IRComponent;
    use alloc::string::ToString;

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
        let value = Value::Integer("1256".to_string(), Type::Integer(32));
        assert_eq!(value.emit(), "1256");
    }
}
