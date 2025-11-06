mod aggregate;
mod binop;
mod block;
mod flow;
mod memory;
mod trunc;
mod unop;

use crate::types::Type;
use crate::value::Value;
use crate::IRComponent;
use std::boxed::Box;
use std::string::String;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::vec::Vec;

/// Represents an instruction in LLVM IR.
pub trait Instruction: IRComponent {}

/// Represents a basic block in LLVM IR.
pub struct BasicBlock {
    basic_block_index: Arc<AtomicUsize>,
    ssa_register_index: Arc<AtomicUsize>,
    label: String,
    instructions: Vec<Box<dyn Instruction + Send>>,
    pub(crate) children: Vec<SharedBasicBlock>,
}

/// A basic block with multiple owners.
#[derive(Clone)]
pub struct SharedBasicBlock {
    inner: Arc<Mutex<BasicBlock>>,
}

impl SharedBasicBlock {
    /// Creates a new shared basic block.
    pub fn new(bb: BasicBlock) -> Self {
        SharedBasicBlock {
            inner: Arc::new(Mutex::new(bb)),
        }
    }

    /// Gain temporary access to the data inside.
    pub fn unlock<F: FnOnce(MutexGuard<BasicBlock>)>(&self, f: F) {
        let inner = self.inner.lock().unwrap();
        f(inner);
    }

    /// Gain temporary access to the data inside, and get a value out.
    pub fn unlock_out<T, F: FnOnce(MutexGuard<BasicBlock>) -> T>(&self, f: F) -> T {
        let inner = self.inner.lock().unwrap();
        f(inner)
    }

    /// Pushes a new instruction into this block.
    pub fn push_instruction(&self, instruction: Box<dyn Instruction + Send>) {
        let mut inner = self.inner.lock().unwrap();
        inner.instructions.push(instruction);
    }

    /// Creates a local register
    pub fn create_local_register(&self, ty: Type) -> (String, Value) {
        let idx = self
            .unlock_out(|x| x.basic_block_index.clone())
            .fetch_add(1, Ordering::AcqRel);
        (
            format!("r{idx}"),
            Value::LocalIdentifier(format!("r{idx}"), ty),
        )
    }

    /// Creates a child of the block.
    pub fn create_child<F: FnOnce(SharedBasicBlock)>(&self, f: F) -> String {
        let bb = BasicBlock::child(self);
        f(bb.clone());
        bb.unlock_out(|x| x.label.clone()).clone()
    }
}
