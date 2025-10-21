use crate::instruction::BasicBlock;
use crate::module::{LinkageType, ModuleComponent};
use crate::types::FunctionPtr;
use crate::value::GlobalIdentifier;
use crate::IRComponent;
use alloc::string::String;
use alloc::vec::Vec;

/// Global variables define regions of memory allocated at compilation time instead of run-time.
pub struct GlobalFunction {
    /// The name of the function.
    name: GlobalIdentifier,
    ty: FunctionPtr,
    /// The linkage type of the function, defaults to LinkageType::External
    linkage: Option<LinkageType>,
    body: Option<FunctionBody>,
}

impl GlobalFunction {
    pub fn new(name: GlobalIdentifier, ty: FunctionPtr) -> Self {
        GlobalFunction {
            name,
            ty,
            linkage: None,
            body: None,
        }
    }

    pub fn linkage(mut self, linkage: LinkageType) -> Self {
        self.linkage = Some(linkage);
        self
    }

    pub fn body(mut self, body: FunctionBody) -> Self {
        self.body = Some(body);
        self
    }
}

impl IRComponent for GlobalFunction {
    fn append_to_string(&self, string: &mut String) {
        string.push_str("define ");
        self.ty.return_type.append_to_string(string);
        string.push(' ');
        self.name.append_to_string(string);
        string.push('(');
        string.push_str(
            &*self
                .ty
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

pub struct FunctionBody {
    entry: BasicBlock,
}

impl FunctionBody {
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
