mod aggregate;
mod binop;
mod block;
mod flow;
mod memory;
mod unop;

use crate::IRComponent;
use std::boxed::Box;
use std::ops::Deref;
use std::string::String;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::vec::Vec;

/// Represents an instruction in LLVM IR.
pub trait Instruction: IRComponent {}

/// Represents a basic block in LLVM IR.
pub struct BasicBlock {
    basic_block_index: Arc<AtomicUsize>,
    ssa_register_index: Arc<AtomicUsize>,
    label: String,
    instructions: Vec<Box<dyn Instruction>>,
    pub(crate) children: Vec<BasicBlock>,
}

/// Represents a pointer to a basic block.
#[derive(Clone)]
pub struct BasicBlockHandle<'a>(pub(crate) &'a BasicBlock);

impl Deref for BasicBlockHandle<'_> {
    type Target = BasicBlock;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
