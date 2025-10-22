use crate::instruction::BasicBlock;
use crate::module::{LinkageType, ModuleComponent};
use crate::types::Type;
use crate::IRComponent;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Global variables define regions of memory allocated at compilation time instead of run-time.
pub struct GlobalFunction {
    /// The name of the function.
    name: String,
    return_type: Type,
    parameters: Vec<Type>,
    /// The linkage type of the function, defaults to LinkageType::External
    linkage: Option<LinkageType>,
    body: Option<FunctionBody>,
}

impl GlobalFunction {
    /// Creates a new global function from a name and signature.
    pub fn new(name: &str, return_type: Type) -> Self {
        GlobalFunction {
            name: name.to_string(),
            return_type,
            parameters: Vec::new(),
            linkage: None,
            body: None,
        }
    }

    /// Adds a parameter to the function.
    pub fn with_parameter(mut self, parameter: Type) -> Self {
        self.parameters.push(parameter);
        self
    }

    /// Defines the linkage type of the global function.
    pub fn linkage(mut self, linkage: LinkageType) -> Self {
        self.linkage = Some(linkage);
        self
    }

    /// Defines the body of the global function.
    pub fn body(mut self, body: FunctionBody) -> Self {
        self.body = Some(body);
        self
    }
}

impl IRComponent for GlobalFunction {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("define ");
        self.return_type.append_to_string(string);
        string.push(' ');
        string.push('@');
        string.push_str(&self.name);
        string.push('(');
        string.push_str(
            &*self
                .parameters
                .iter()
                .map(|p| p.emit())
                .collect::<Vec<_>>()
                .join(" "),
        );
        string.push(')');
        string.push(' ');
        self.body.iter().for_each(|body| {
            string.push('{');
            string.push(' ');
            body.entry.append_to_string(string);
            string.push('}');
        })
    }
}

impl ModuleComponent for GlobalFunction {}

/// Represents a function body in LLVM IR.
pub struct FunctionBody {
    entry: BasicBlock,
}

impl FunctionBody {
    /// Creates a new function body. This gives you a reference to a Basic Block, which is
    /// the entrypoint of the function body.
    pub fn new<F: FnOnce(&mut BasicBlock)>(handler: F) -> Self {
        let mut f = FunctionBody {
            entry: BasicBlock::entry("entry"),
        };
        handler(&mut f.entry);
        f
    }
}

impl IRComponent for FunctionBody {
    fn append_to_string(&self, string: &mut String) {
        string.push('{');
        self.entry.append_to_string(string);
        string.push('}');
    }
}
