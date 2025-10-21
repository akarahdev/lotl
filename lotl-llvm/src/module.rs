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

#[cfg(test)]
mod tests {
    use crate::module::GlobalVariable;
    use crate::module::LinkageType;
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
