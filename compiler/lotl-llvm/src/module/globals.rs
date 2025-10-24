use crate::module::{LinkageType, ModuleComponent};
use crate::types::Type;
use crate::value::Value;
use crate::IRComponent;
use alloc::string::{String, ToString};

/// Global variables define regions of memory allocated at compilation time instead of run-time.
pub struct GlobalVariable {
    /// The name of the global variable.
    name: String,
    /// The linkage type of the value, defaults to LinkageType::External
    linkage: Option<LinkageType>,
    /// The type of the global variable.
    ty: Type,
    /// The default value of the global variable.
    value: Option<Value>,
}

impl GlobalVariable {
    /// Creates a new global variable with a name and a type.
    pub fn new(name: &str, ty: Type) -> GlobalVariable {
        GlobalVariable {
            name: name.to_string(),
            ty,
            linkage: None,
            value: None,
        }
    }

    /// Provides a linkage type to the global variable.
    pub fn with_linkage(mut self, linkage: LinkageType) -> Self {
        self.linkage = Some(linkage);
        self
    }

    /// Provides a value to the global variable.
    pub fn with_value(mut self, value: Value) -> Self {
        self.value = Some(value);
        self
    }
}

impl IRComponent for GlobalVariable {
    fn append_to_string(&self, string: &mut String) {
        string.push('@');
        string.push_str(&self.name);
        string.push_str(" = ");
        self.linkage.iter().for_each(|e| {
            e.append_to_string(string);
            string.push(' ');
        });
        string.push_str("global ");
        self.ty.append_to_string(string);
        string.push(' ');
        self.value.iter().for_each(|e| {
            e.append_to_string(string);
        });
    }
}

impl ModuleComponent for GlobalVariable {}
