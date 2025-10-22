use crate::IRComponent;
use alloc::string::String;

mod function;
mod globals;

pub use function::*;
pub use globals::*;

/// Represents an LLVM module.
///
/// LLVM programs are composed of Modules, each of which is a translation unit of the input programs.
/// Each module consists of functions, global variables, and symbol table entries.
pub struct Module {}

/// Represents the Linkage Type of Global Variables and Function.
pub enum LinkageType {
    /// Global values with this linkage are only directly accessible by objects in the current
    /// module.
    Private,
    /// Similar to private, but the value shows as a local symbol in the object file.
    Internal,
    /// Globals with this linkage are never emitted into the object file
    /// corresponding to the LLVM module. From the linker’s perspective, a
    /// global with this linkage type is equivalent to an external declaration.
    ///
    /// This linkage type is only allowed on definitions, not declarations.
    AvailableExternally,
    /// This means the object participates in linkage and can be used to resolve external dependencies.
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

#[cfg(test)]
mod tests {
    use crate::module::GlobalVariable;
    use crate::module::LinkageType;
    use crate::types::Type;
    use crate::value::Value;
    use crate::IRComponent;
    use alloc::string::ToString;

    #[test]
    pub fn generate_simple_global_variable() {
        let var = GlobalVariable::new("foo", Type::Integer(32))
            .with_linkage(LinkageType::Internal)
            .with_value(Value::Integer("1240".to_string(), Type::Integer(32)));
        assert_eq!(var.emit(), "@foo = internal global i32 1240");
    }
}
