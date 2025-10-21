use alloc::boxed::Box;
use alloc::string::String;
use crate::IRComponent;
use crate::module::{LinkageType, ModuleComponent};
use crate::types::Type;
use crate::value::{Constant, GlobalIdentifier};

/// Global variables define regions of memory allocated at compilation time instead of run-time.
pub struct GlobalVariable {
    /// The name of the global variable.
    name: GlobalIdentifier,
    /// The linkage type of the value, defaults to LinkageType::External
    linkage: Option<LinkageType>,
    /// The type of the global variable.
    ty: Box<dyn Type>,
    /// The default value of the global variable.
    value: Option<Box<dyn Constant>>,
}

impl GlobalVariable {
    pub fn new<T: Type + 'static>(name: GlobalIdentifier, ty: T) -> GlobalVariable {
        GlobalVariable {
            name,
            ty: Box::new(ty),
            linkage: None,
            value: None,
        }
    }

    pub fn with_linkage(mut self, linkage: LinkageType) -> Self {
        self.linkage = Some(linkage);
        self
    }
    pub fn with_value<C: Constant + 'static>(mut self, value: C) -> Self {
        self.value = Some(Box::new(value));
        self
    }
}

impl IRComponent for GlobalVariable {
    fn append_to_string(&self, string: &mut String) {
        self.name.append_to_string(string);
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
