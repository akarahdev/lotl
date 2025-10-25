mod aggregate;
mod binop;
mod block;
mod flow;
mod memory;
mod unop;

use crate::IRComponent;
use std::boxed::Box;
use std::ops::{Deref, DerefMut};
use std::string::String;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::vec::Vec;

pub trait Instruction: IRComponent {}

pub struct BasicBlock {
    basic_block_index: Arc<AtomicUsize>,
    ssa_register_index: Arc<AtomicUsize>,
    label: String,
    instructions: Vec<Box<dyn Instruction>>,
    pub(crate) children: Vec<BasicBlock>,
}

pub struct BasicBlockHandle<'a>(pub(crate) &'a mut BasicBlock);

impl Deref for BasicBlockHandle<'_> {
    type Target = BasicBlock;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl DerefMut for BasicBlockHandle<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
