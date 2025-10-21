use crate::types::Type;
use crate::value::{Constant, GlobalIdentifier};
use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::String;

/// Represents an LLVM module.
///
/// LLVM programs are composed of Modules, each of which is a translation unit of the input programs.
/// Each module consists of functions, global variables, and symbol table entries.
pub struct Module {}

/// Represents the Linkage Type of Global Variables and Function.
pub enum LinkageType {
    /// Global values with “private” linkage are only directly accessible by objects in the current
    /// module.
    ///
    /// In particular, linking code into a module with a private global value may cause the private
    /// to be renamed as necessary to avoid collisions.
    ///
    /// Because the symbol is private to the module, all references can be updated.
    ///
    /// This doesn’t show up in any symbol table in the object file.
    Private,
    /// Similar to private, but the value shows as a local symbol (STB_LOCAL in the case of ELF)
    /// in the object file.
    ///
    /// This corresponds to the notion of the ‘static’ keyword in C.
    Internal,
    /// Globals with “available_externally” linkage are never emitted into the object file
    /// corresponding to the LLVM module. From the linker’s perspective, an available_externally
    /// global is equivalent to an external declaration.
    ///
    /// They exist to allow inlining and other optimizations to take place given knowledge of
    /// the definition of the global, which is known to be somewhere outside the module.
    ///
    /// Globals with available_externally linkage are allowed to be discarded at will,
    /// and allow inlining and other optimizations.
    ///
    /// This linkage type is only allowed on definitions, not declarations.
    AvailableExternally,
    /// This is the default linkage type, meaning that  it participates in linkage and
    /// can be used to resolve external symbol references.
    External,
}

impl IRComponent for LinkageType {
    fn append_to_string(&self, string: &mut String) {
        match self {
            LinkageType::Private => string.push_str("private"),
            LinkageType::Internal => string.push_str("internal"),
            LinkageType::AvailableExternally => string.push_str("available_externally"),
            LinkageType::External => string.push_str(""),
        }
    }
}

/// A marker trait, indicating this is a valid top-level component of a module.
pub trait ModuleComponent: IRComponent {}

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

/// Global variables define regions of memory allocated at compilation time instead of run-time.
pub struct GlobalFunction {
    /// The name of the function.
    name: GlobalIdentifier,
    /// The linkage type of the function, defaults to LinkageType::External
    linkage: Option<LinkageType>,
}

impl GlobalFunction {
    pub fn new(name: GlobalIdentifier) -> Self {
        GlobalFunction {
            name,
            linkage: None,
        }
    }

    pub fn linkage(mut self, linkage: LinkageType) -> Self {
        self.linkage = Some(linkage);
        self
    }
}

impl IRComponent for GlobalFunction {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("define void ");
        self.name.append_to_string(string);
        string.push('(');
        string.push(')');
    }
}

impl ModuleComponent for GlobalFunction {}

#[cfg(test)]
mod tests {
    use crate::module::{GlobalVariable, LinkageType};
    use crate::types::Types;
    use crate::value::Values;
    use crate::IRComponent;

    #[test]
    pub fn generate_simple_global_variable() {
        let var = GlobalVariable::new(Values::global("foo"), Types::integer(32))
            .with_linkage(LinkageType::Internal)
            .with_value(Values::integer("1240").unwrap());
        assert_eq!(var.emit(), "@foo = internal global i32 1240");
    }
}
