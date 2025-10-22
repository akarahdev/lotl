use crate::IRComponent;
use alloc::string::String;

/// Represents a valid LLVM value.
#[allow(unused)]
pub trait Value: IRComponent {}

/// A structure holding methods constructing any valid LLVM value.
pub struct Values;

impl Values {
    /// Creates a constant integer that can be used as a value.
    pub fn integer(value: &str) -> Option<impl Constant>
    where
        Self: Sized,
    {
        for char in value.chars() {
            if !char.is_digit(10) {
                return None;
            }
        }
        Some(Integer {
            value: String::from(value),
        })
    }

    /// Creates a constant global identifier.
    pub fn global(name: &str) -> GlobalIdentifier
    where
        Self: Sized,
    {
        GlobalIdentifier {
            name: String::from(name),
        }
    }

    /// Creates a constant local identifier.
    pub fn local(name: &str) -> LocalIdentifier
    where
        Self: Sized,
    {
        LocalIdentifier {
            name: String::from(name),
        }
    }
}

/// LLVM identifiers come in two basic types: global and local.
/// Global identifiers (functions, global variables) begin with the '@' character.
#[derive(Clone)]
pub struct GlobalIdentifier {
    /// The name of the identifier
    name: String,
}

impl IRComponent for GlobalIdentifier {
    fn append_to_string(&self, string: &mut String) {
        string.push('@');
        string.push_str(&self.name);
    }
}
impl Value for GlobalIdentifier {}

/// LLVM identifiers come in two basic types: global and local.
/// Local identifiers (register names, types) begin with the '%' character.
#[derive(Clone)]
pub struct LocalIdentifier {
    /// The name of the identifier
    pub name: String,
}

impl IRComponent for LocalIdentifier {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.name);
    }
}
impl Value for LocalIdentifier {}

/// Represents a valid LLVM IR constant.
pub trait Constant: Value {}

struct Integer {
    value: String,
}

impl IRComponent for Integer {
    fn append_to_string(&self, string: &mut String) {
        string.push_str(&self.value);
    }
}

impl Constant for Integer {}

impl Value for Integer {}

#[cfg(test)]
mod tests {
    use crate::value::Values;
    use crate::IRComponent;

    #[test]
    pub fn test_local_idents() {
        let value = Values::local("foo");
        assert_eq!(value.emit(), "%foo");
    }
    #[test]
    pub fn test_global_idents() {
        let value = Values::global("bar");
        assert_eq!(value.emit(), "@bar");
    }
    #[test]
    pub fn test_int_constants() {
        let value = Values::integer("1256").unwrap();
        assert_eq!(value.emit(), "1256");
    }
}
