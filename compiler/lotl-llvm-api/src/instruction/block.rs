use crate::instruction::{BasicBlock, SharedBasicBlock};
use crate::types::Type;
use crate::value::Value;
use crate::IRComponent;
use std::format;
use std::string::String;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::vec::Vec;

impl BasicBlock {
    /// Creates a new basic block with the given label, that is the entry to a function.
    pub fn entry(label: &str) -> BasicBlock {
        BasicBlock {
            basic_block_index: Arc::new(AtomicUsize::new(0)),
            ssa_register_index: Arc::new(AtomicUsize::new(0)),
            label: String::from(label),
            instructions: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Creates a child of the associated block, and returns it.
    pub fn child(parent: &SharedBasicBlock) -> SharedBasicBlock {
        let out = SharedBasicBlock::new(BasicBlock {
            label: format!(
                "bb{}",
                parent
                    .unlock_out(|x| x.basic_block_index.clone())
                    .fetch_add(1, Ordering::AcqRel)
            ),
            basic_block_index: parent.unlock_out(|x| x.basic_block_index.clone()).clone(),
            ssa_register_index: parent.unlock_out(|x| x.ssa_register_index.clone()).clone(),
            instructions: Vec::new(),
            children: Vec::new(),
        });
        parent.unlock(|mut x| x.children.push(out.clone()));
        out
    }

    pub(crate) fn create_local_register(&self, ty: Type) -> (String, Value) {
        let idx = self.basic_block_index.fetch_add(1, Ordering::AcqRel);
        (
            format!("r{idx}"),
            Value::LocalIdentifier(format!("r{idx}"), ty),
        )
    }
}

impl IRComponent for BasicBlock {
    fn append_to_string(&self, string: &mut String) {
        string.push_str(self.label.as_str());
        string.push(':');
        string.push(' ');
        string.push_str(
            &self
                .instructions
                .iter()
                .map(|x| x.emit())
                .collect::<Vec<_>>()
                .join(" "),
        );
        string.push(' ');

        for child in &self.children {
            child.unlock_out(|child| child.append_to_string(string));
        }
    }
}
