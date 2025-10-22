use crate::instruction::BasicBlock;
use crate::types::Type;
use crate::value::Value;
use crate::IRComponent;
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

impl BasicBlock {
    pub fn entry(label: &str) -> BasicBlock {
        BasicBlock {
            basic_block_index: Arc::new(AtomicUsize::new(0)),
            ssa_register_index: Arc::new(AtomicUsize::new(0)),
            label: String::from(label),
            instructions: Vec::new(),
            children: Vec::new(),
        }
    }

    pub(crate) fn child(parent: &BasicBlock) -> BasicBlock {
        BasicBlock {
            label: format!(
                "bb{}",
                parent.basic_block_index.fetch_add(1, Ordering::AcqRel)
            ),
            basic_block_index: parent.basic_block_index.clone(),
            ssa_register_index: parent.ssa_register_index.clone(),
            instructions: Vec::new(),
            children: Vec::new(),
        }
    }

    pub(crate) fn create_child<F: FnOnce(&mut BasicBlock)>(&mut self, f: F) -> String {
        let mut bb = BasicBlock::child(self);
        f(&mut bb);
        let label = bb.label.clone();
        self.children.push(bb);
        label
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
            child.append_to_string(string);
        }
    }
}
